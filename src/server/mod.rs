use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
use serde_json;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::io;
use url::form_urlencoded;
#[allow(unused_imports)]
use swagger;
use swagger::{ApiError, XSpanIdString, Has, RequestParser};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use swagger::context::ContextualPayload;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

use crate::{Api,
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

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
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
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_PCA9956B_API: usize = 0;
    pub(crate) static ID_PCA9956B_BUSID_RESET: usize = 1;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_RESET: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/reset$")
                .expect("Unable to create regex for PCA9956B_BUSID_RESET");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ADDR_NUM: usize = 2;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ADDR_NUM");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR: usize = 3;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/addr$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL: usize = 4;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/addr/(?P<addrVal>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED: usize = 5;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/enabled$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED: usize = 6;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/addr/(?P<num>[^/?#]*)/enabled/(?P<enabled>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_CONFIG: usize = 7;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_CONFIG: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/config$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_CONFIG");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_CURRENT: usize = 8;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_CURRENT: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/current$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_CURRENT");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_CURRENT_CURRENT: usize = 9;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_CURRENT_CURRENT: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/current/(?P<current>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_CURRENT_CURRENT");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ERROR: usize = 10;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ERROR: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/error$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ERROR");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ERROR_CLEAR: usize = 11;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ERROR_CLEAR: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/error/clear$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ERROR_CLEAR");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_ERRORS: usize = 12;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_ERRORS: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/errors$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_ERRORS");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_FREQ: usize = 13;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_FREQ: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/freq$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_FREQ");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_FREQ_FREQ: usize = 14;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_FREQ_FREQ: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/freq/(?P<freq>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_FREQ_FREQ");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_GROUP: usize = 15;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_GROUP: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/group$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_GROUP");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_GROUP_GROUP: usize = 16;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_GROUP_GROUP: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/group/(?P<group>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_GROUP_GROUP");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED: usize = 17;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED: usize = 18;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT: usize = 19;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/current$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_CURRENT");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT: usize = 20;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/current/(?P<current>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR: usize = 21;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/error$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_ERROR");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR: usize = 22;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/error/(?P<error>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_PWM: usize = 23;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/pwm$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_PWM");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM: usize = 24;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/pwm/(?P<pwm>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_STATE: usize = 25;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/state$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_STATE");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE: usize = 26;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/led/(?P<led>[^/?#]*)/state/(?P<state>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_OFFSET: usize = 27;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OFFSET: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/offset$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_OFFSET");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_OFFSET_OFFSET: usize = 28;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OFFSET_OFFSET: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/offset/(?P<offset>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_OFFSET_OFFSET");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE: usize = 29;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/outputChange$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_OUTPUTCHANGE");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE: usize = 30;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/outputChange/(?P<outputChange>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_OVERTEMP: usize = 31;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_OVERTEMP: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/overTemp$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_OVERTEMP");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_PWM: usize = 32;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_PWM: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/pwm$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_PWM");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_PWM_PWM: usize = 33;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_PWM_PWM: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/pwm/(?P<pwm>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_PWM_PWM");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_SLEEP: usize = 34;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_SLEEP: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/sleep$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_SLEEP");
    }
    pub(crate) static ID_PCA9956B_BUSID_ADDR_SLEEP_SLEEP: usize = 35;
    lazy_static! {
        pub static ref REGEX_PCA9956B_BUSID_ADDR_SLEEP_SLEEP: regex::Regex =
            regex::Regex::new(r"^/pca9956b/(?P<busId>[^/?#]*)/(?P<addr>[^/?#]*)/sleep/(?P<sleep>[^/?#]*)$")
                .expect("Unable to create regex for PCA9956B_BUSID_ADDR_SLEEP_SLEEP");
    }
}

pub struct MakeService<T, RC> {
    api_impl: T,
    marker: PhantomData<RC>,
}

impl<T, RC> MakeService<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<'a, T, SC, RC> hyper::service::MakeService<&'a SC> for MakeService<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static + Send
{
    type ReqBody = ContextualPayload<Body, RC>;
    type ResBody = Body;
    type Error = Error;
    type Service = Service<T, RC>;
    type Future = future::FutureResult<Self::Service, Self::MakeError>;
    type MakeError = Error;

    fn make_service(&mut self, _ctx: &'a SC) -> Self::Future {
        future::FutureResult::from(Ok(Service::new(
            self.api_impl.clone(),
        )))
    }
}

type ServiceFuture = Box<dyn Future<Item = Response<Body>, Error = Error> + Send>;

fn method_not_allowed() -> ServiceFuture {
    Box::new(future::ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    ))
}

pub struct Service<T, RC> {
    api_impl: T,
    marker: PhantomData<RC>,
}

impl<T, RC> Service<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static {
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> hyper::service::Service for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + 'static + Send
{
    type ReqBody = ContextualPayload<Body, C>;
    type ResBody = Body;
    type Error = Error;
    type Future = ServiceFuture;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (parts, body) = req.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());
        let mut context = body.context;
        let body = body.inner;

        match &method {

            // ClearError - POST /pca9956b/{busId}/{addr}/error/clear
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR_CLEAR) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ERROR_CLEAR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ERROR_CLEAR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ERROR_CLEAR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.clear_error(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ClearErrorResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                ClearErrorResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CLEAR_ERROR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ClearErrorResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CLEAR_ERROR_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetAddrEnabled - GET /pca9956b/{busId}/{addr}/addr/{num}/enabled
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter num: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_addr_enabled(
                                            param_bus_id,
                                            param_addr,
                                            param_num,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAddrEnabledResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_ENABLED_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAddrEnabledResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_ENABLED_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAddrEnabledResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_ENABLED_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetAddrInfo - GET /pca9956b/{busId}/{addr}/addr/{num}
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter num: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_addr_info(
                                            param_bus_id,
                                            param_addr,
                                            param_num,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAddrInfoResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_INFO_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAddrInfoResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_INFO_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAddrInfoResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_INFO_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetAddrValue - GET /pca9956b/{busId}/{addr}/addr/{num}/addr
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter num: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_addr_value(
                                            param_bus_id,
                                            param_addr,
                                            param_num,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAddrValueResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_VALUE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAddrValueResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_VALUE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAddrValueResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ADDR_VALUE_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetApi - GET /pca9956b/api
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_API) => {
                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_api(
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetApiResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/x-yaml")
                                                            .expect("Unable to create Content-Type header for GET_API_OK"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetApiResponse::FileNotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for GET_API_FILE_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetConfig - GET /pca9956b/{busId}/{addr}/config
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CONFIG in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_config(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetConfigResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetConfigResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetConfigResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetCurrent - GET /pca9956b/{busId}/{addr}/current
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_current(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCurrentResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CURRENT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCurrentResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CURRENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCurrentResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CURRENT_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetError - GET /pca9956b/{busId}/{addr}/error
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ERROR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ERROR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ERROR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_error(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetErrorResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ERROR_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetErrorResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ERROR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetErrorResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ERROR_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetErrors - POST /pca9956b/{busId}/{addr}/errors
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERRORS) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ERRORS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ERRORS in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ERRORS.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_errors(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetErrorsResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ERRORS_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetErrorsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ERRORS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetErrorsResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ERRORS_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetFreq - GET /pca9956b/{busId}/{addr}/freq
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_FREQ
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_FREQ in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_FREQ.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_freq(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetFreqResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_FREQ_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetFreqResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_FREQ_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetFreqResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_FREQ_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetGroup - GET /pca9956b/{busId}/{addr}/group
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_GROUP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_GROUP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_GROUP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_group(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetGroupResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GROUP_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetGroupResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GROUP_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetGroupResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GROUP_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetLedCurrent - GET /pca9956b/{busId}/{addr}/led/{led}/current
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_led_current(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedCurrentResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_CURRENT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedCurrentResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_CURRENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedCurrentResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_CURRENT_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetLedError - GET /pca9956b/{busId}/{addr}/led/{led}/error
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_ERROR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_led_error(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedErrorResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_ERROR_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedErrorResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_ERROR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedErrorResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_ERROR_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetLedInfo - GET /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_led_info(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedInfoResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_INFO_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedInfoResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_INFO_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedInfoResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_INFO_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetLedInfoAll - GET /pca9956b/{busId}/{addr}/led
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_led_info_all(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedInfoAllResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_INFO_ALL_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedInfoAllResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_INFO_ALL_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedInfoAllResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_INFO_ALL_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetLedPwm - GET /pca9956b/{busId}/{addr}/led/{led}/pwm
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_led_pwm(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedPwmResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_PWM_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedPwmResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_PWM_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedPwmResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_PWM_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetLedState - GET /pca9956b/{busId}/{addr}/led/{led}/state
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_STATE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_led_state(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLedStateResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_STATE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedStateResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_STATE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLedStateResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LED_STATE_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetOffset - GET /pca9956b/{busId}/{addr}/offset
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OFFSET in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_offset(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOffsetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OFFSET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOffsetResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OFFSET_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOffsetResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OFFSET_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetOutputChange - GET /pca9956b/{busId}/{addr}/outputChange
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OUTPUTCHANGE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_output_change(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOutputChangeResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OUTPUT_CHANGE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOutputChangeResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OUTPUT_CHANGE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOutputChangeResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OUTPUT_CHANGE_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetOverTemp - GET /pca9956b/{busId}/{addr}/overTemp
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OVERTEMP) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OVERTEMP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OVERTEMP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OVERTEMP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_over_temp(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOverTempResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OVER_TEMP_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOverTempResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OVER_TEMP_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOverTempResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_OVER_TEMP_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetPwm - GET /pca9956b/{busId}/{addr}/pwm
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_pwm(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetPwmResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_PWM_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetPwmResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_PWM_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetPwmResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_PWM_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // GetSleep - GET /pca9956b/{busId}/{addr}/sleep
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_SLEEP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get_sleep(
                                            param_bus_id,
                                            param_addr,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetSleepResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SLEEP_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetSleepResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SLEEP_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetSleepResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_SLEEP_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // Reset - POST /pca9956b/{busId}/reset
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_RESET) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_RESET
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_RESET in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_RESET.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.reset(
                                            param_bus_id,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ResetResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                ResetResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RESET_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ResetResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RESET_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetAddrEnabled - POST /pca9956b/{busId}/{addr}/addr/{num}/enabled/{enabled}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter num: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_enabled = match percent_encoding::percent_decode(path_params["enabled"].as_bytes()).decode_utf8() {
                    Ok(param_enabled) => match param_enabled.parse::<bool>() {
                        Ok(param_enabled) => param_enabled,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter enabled: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["enabled"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_addr_enabled(
                                            param_bus_id,
                                            param_addr,
                                            param_num,
                                            param_enabled,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetAddrEnabledResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetAddrEnabledResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_ADDR_ENABLED_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetAddrEnabledResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_ADDR_ENABLED_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetAddrValue - POST /pca9956b/{busId}/{addr}/addr/{num}/addr/{addrVal}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_num = match percent_encoding::percent_decode(path_params["num"].as_bytes()).decode_utf8() {
                    Ok(param_num) => match param_num.parse::<i32>() {
                        Ok(param_num) => param_num,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter num: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["num"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr_val = match percent_encoding::percent_decode(path_params["addrVal"].as_bytes()).decode_utf8() {
                    Ok(param_addr_val) => match param_addr_val.parse::<i32>() {
                        Ok(param_addr_val) => param_addr_val,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addrVal: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addrVal"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_addr_value(
                                            param_bus_id,
                                            param_addr,
                                            param_num,
                                            param_addr_val,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetAddrValueResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetAddrValueResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_ADDR_VALUE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetAddrValueResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_ADDR_VALUE_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetConfig - POST /pca9956b/{busId}/{addr}/config
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CONFIG in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CONFIG.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Self::Future {
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
                                        Err(e) => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Config - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Config due to schema"))),
                                    }
                                } else {
                                    None
                                };
                                let param_config = match param_config {
                                    Some(param_config) => param_config,
                                    None => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Config"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Config"))),
                                };

                                Box::new(
                                    api_impl.set_config(
                                            param_bus_id,
                                            param_addr,
                                            param_config,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetConfigResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetConfigResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_CONFIG_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetConfigResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_CONFIG_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                            },
                            Err(e) => Box::new(future::ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Config: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Config"))),
                        }
                    })
                ) as Self::Future
            },

            // SetCurrent - POST /pca9956b/{busId}/{addr}/current/{current}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT_CURRENT) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_CURRENT_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_CURRENT_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_current = match percent_encoding::percent_decode(path_params["current"].as_bytes()).decode_utf8() {
                    Ok(param_current) => match param_current.parse::<i32>() {
                        Ok(param_current) => param_current,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter current: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["current"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_current(
                                            param_bus_id,
                                            param_addr,
                                            param_current,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetCurrentResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetCurrentResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_CURRENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetCurrentResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_CURRENT_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetFreq - POST /pca9956b/{busId}/{addr}/freq/{freq}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ_FREQ) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_FREQ_FREQ
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_FREQ_FREQ in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_FREQ_FREQ.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_freq = match percent_encoding::percent_decode(path_params["freq"].as_bytes()).decode_utf8() {
                    Ok(param_freq) => match param_freq.parse::<i32>() {
                        Ok(param_freq) => param_freq,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter freq: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["freq"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_freq(
                                            param_bus_id,
                                            param_addr,
                                            param_freq,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetFreqResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetFreqResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_FREQ_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetFreqResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_FREQ_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetGroup - POST /pca9956b/{busId}/{addr}/group/{group}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP_GROUP) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_GROUP_GROUP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_GROUP_GROUP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_GROUP_GROUP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_group = match percent_encoding::percent_decode(path_params["group"].as_bytes()).decode_utf8() {
                    Ok(param_group) => match param_group.parse::<models::Group>() {
                        Ok(param_group) => param_group,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter group: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["group"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_group(
                                            param_bus_id,
                                            param_addr,
                                            param_group,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetGroupResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetGroupResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_GROUP_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetGroupResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_GROUP_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetLedCurrent - POST /pca9956b/{busId}/{addr}/led/{led}/current/{current}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_current = match percent_encoding::percent_decode(path_params["current"].as_bytes()).decode_utf8() {
                    Ok(param_current) => match param_current.parse::<i32>() {
                        Ok(param_current) => param_current,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter current: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["current"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_led_current(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                            param_current,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedCurrentResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetLedCurrentResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_CURRENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetLedCurrentResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_CURRENT_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetLedError - POST /pca9956b/{busId}/{addr}/led/{led}/error/{error}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_error = match percent_encoding::percent_decode(path_params["error"].as_bytes()).decode_utf8() {
                    Ok(param_error) => match param_error.parse::<models::LedError>() {
                        Ok(param_error) => param_error,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter error: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["error"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_led_error(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                            param_error,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedErrorResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetLedErrorResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_ERROR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetLedErrorResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_ERROR_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetLedInfo - POST /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Self::Future {
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
                                        Err(e) => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter LedInfo - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter LedInfo due to schema"))),
                                    }
                                } else {
                                    None
                                };
                                let param_led_info = match param_led_info {
                                    Some(param_led_info) => param_led_info,
                                    None => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter LedInfo"))
                                                        .expect("Unable to create Bad Request response for missing body parameter LedInfo"))),
                                };

                                Box::new(
                                    api_impl.set_led_info(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                            param_led_info,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedInfoResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetLedInfoResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_INFO_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetLedInfoResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_INFO_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                            },
                            Err(e) => Box::new(future::ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter LedInfo: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter LedInfo"))),
                        }
                    })
                ) as Self::Future
            },

            // SetLedInfoAll - POST /pca9956b/{busId}/{addr}/led
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Self::Future {
                        match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_led_info_array: Option<models::LedInfoArray> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_led_info_array) => param_led_info_array,
                                        Err(e) => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter LedInfoArray - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter LedInfoArray due to schema"))),
                                    }
                                } else {
                                    None
                                };
                                let param_led_info_array = match param_led_info_array {
                                    Some(param_led_info_array) => param_led_info_array,
                                    None => return Box::new(future::ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter LedInfoArray"))
                                                        .expect("Unable to create Bad Request response for missing body parameter LedInfoArray"))),
                                };

                                Box::new(
                                    api_impl.set_led_info_all(
                                            param_bus_id,
                                            param_addr,
                                            param_led_info_array,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedInfoAllResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetLedInfoAllResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_INFO_ALL_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetLedInfoAllResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_INFO_ALL_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                            },
                            Err(e) => Box::new(future::ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter LedInfoArray: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter LedInfoArray"))),
                        }
                    })
                ) as Self::Future
            },

            // SetLedPwm - POST /pca9956b/{busId}/{addr}/led/{led}/pwm/{pwm}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_pwm = match percent_encoding::percent_decode(path_params["pwm"].as_bytes()).decode_utf8() {
                    Ok(param_pwm) => match param_pwm.parse::<i32>() {
                        Ok(param_pwm) => param_pwm,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter pwm: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["pwm"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_led_pwm(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                            param_pwm,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedPwmResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetLedPwmResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_PWM_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetLedPwmResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_PWM_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetLedState - POST /pca9956b/{busId}/{addr}/led/{led}/state/{state}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_led = match percent_encoding::percent_decode(path_params["led"].as_bytes()).decode_utf8() {
                    Ok(param_led) => match param_led.parse::<i32>() {
                        Ok(param_led) => param_led,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter led: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["led"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_state = match percent_encoding::percent_decode(path_params["state"].as_bytes()).decode_utf8() {
                    Ok(param_state) => match param_state.parse::<models::LedState>() {
                        Ok(param_state) => param_state,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter state: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["state"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_led_state(
                                            param_bus_id,
                                            param_addr,
                                            param_led,
                                            param_state,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetLedStateResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetLedStateResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_STATE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetLedStateResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_LED_STATE_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetOffset - POST /pca9956b/{busId}/{addr}/offset/{offset}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET_OFFSET) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET_OFFSET
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OFFSET_OFFSET in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OFFSET_OFFSET.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_offset = match percent_encoding::percent_decode(path_params["offset"].as_bytes()).decode_utf8() {
                    Ok(param_offset) => match param_offset.parse::<i32>() {
                        Ok(param_offset) => param_offset,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter offset: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["offset"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_offset(
                                            param_bus_id,
                                            param_addr,
                                            param_offset,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetOffsetResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetOffsetResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_OFFSET_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetOffsetResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_OFFSET_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetOutputChange - POST /pca9956b/{busId}/{addr}/outputChange/{outputChange}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_output_change = match percent_encoding::percent_decode(path_params["outputChange"].as_bytes()).decode_utf8() {
                    Ok(param_output_change) => match param_output_change.parse::<models::OutputChange>() {
                        Ok(param_output_change) => param_output_change,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter outputChange: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["outputChange"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_output_change(
                                            param_bus_id,
                                            param_addr,
                                            param_output_change,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetOutputChangeResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetOutputChangeResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_OUTPUT_CHANGE_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetOutputChangeResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_OUTPUT_CHANGE_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetPwm - POST /pca9956b/{busId}/{addr}/pwm/{pwm}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM_PWM) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_PWM_PWM
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_PWM_PWM in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_PWM_PWM.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_pwm = match percent_encoding::percent_decode(path_params["pwm"].as_bytes()).decode_utf8() {
                    Ok(param_pwm) => match param_pwm.parse::<i32>() {
                        Ok(param_pwm) => param_pwm,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter pwm: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["pwm"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_pwm(
                                            param_bus_id,
                                            param_addr,
                                            param_pwm,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetPwmResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetPwmResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_PWM_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetPwmResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_PWM_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // SetSleep - POST /pca9956b/{busId}/{addr}/sleep/{sleep}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP_SLEEP) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP_SLEEP
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PCA9956B_BUSID_ADDR_SLEEP_SLEEP in set but failed match against \"{}\"", path, paths::REGEX_PCA9956B_BUSID_ADDR_SLEEP_SLEEP.as_str())
                    );

                let param_bus_id = match percent_encoding::percent_decode(path_params["busId"].as_bytes()).decode_utf8() {
                    Ok(param_bus_id) => match param_bus_id.parse::<i32>() {
                        Ok(param_bus_id) => param_bus_id,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter busId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["busId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_addr = match percent_encoding::percent_decode(path_params["addr"].as_bytes()).decode_utf8() {
                    Ok(param_addr) => match param_addr.parse::<i32>() {
                        Ok(param_addr) => param_addr,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter addr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["addr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_sleep = match percent_encoding::percent_decode(path_params["sleep"].as_bytes()).decode_utf8() {
                    Ok(param_sleep) => match param_sleep.parse::<bool>() {
                        Ok(param_sleep) => param_sleep,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter sleep: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["sleep"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.set_sleep(
                                            param_bus_id,
                                            param_addr,
                                            param_sleep,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SetSleepResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                SetSleepResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_SLEEP_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                SetSleepResponse::OperationFailed
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SET_SLEEP_OPERATION_FAILED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            _ if path.matched(paths::ID_PCA9956B_API) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_RESET) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT_CURRENT) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR_CLEAR) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERRORS) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ_FREQ) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP_GROUP) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET_OFFSET) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OVERTEMP) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM_PWM) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP) => method_not_allowed(),
            _ if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP_SLEEP) => method_not_allowed(),
            _ => Box::new(future::ok(
                Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response")
            )) as Self::Future
        }
    }
}

impl<T, C> Clone for Service<T, C> where T: Clone
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
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // ClearError - POST /pca9956b/{busId}/{addr}/error/clear
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR_CLEAR) => Ok("ClearError"),
            // GetAddrEnabled - GET /pca9956b/{busId}/{addr}/addr/{num}/enabled
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED) => Ok("GetAddrEnabled"),
            // GetAddrInfo - GET /pca9956b/{busId}/{addr}/addr/{num}
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM) => Ok("GetAddrInfo"),
            // GetAddrValue - GET /pca9956b/{busId}/{addr}/addr/{num}/addr
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR) => Ok("GetAddrValue"),
            // GetApi - GET /pca9956b/api
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_API) => Ok("GetApi"),
            // GetConfig - GET /pca9956b/{busId}/{addr}/config
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => Ok("GetConfig"),
            // GetCurrent - GET /pca9956b/{busId}/{addr}/current
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT) => Ok("GetCurrent"),
            // GetError - GET /pca9956b/{busId}/{addr}/error
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERROR) => Ok("GetError"),
            // GetErrors - POST /pca9956b/{busId}/{addr}/errors
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ERRORS) => Ok("GetErrors"),
            // GetFreq - GET /pca9956b/{busId}/{addr}/freq
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ) => Ok("GetFreq"),
            // GetGroup - GET /pca9956b/{busId}/{addr}/group
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP) => Ok("GetGroup"),
            // GetLedCurrent - GET /pca9956b/{busId}/{addr}/led/{led}/current
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT) => Ok("GetLedCurrent"),
            // GetLedError - GET /pca9956b/{busId}/{addr}/led/{led}/error
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR) => Ok("GetLedError"),
            // GetLedInfo - GET /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => Ok("GetLedInfo"),
            // GetLedInfoAll - GET /pca9956b/{busId}/{addr}/led
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => Ok("GetLedInfoAll"),
            // GetLedPwm - GET /pca9956b/{busId}/{addr}/led/{led}/pwm
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM) => Ok("GetLedPwm"),
            // GetLedState - GET /pca9956b/{busId}/{addr}/led/{led}/state
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE) => Ok("GetLedState"),
            // GetOffset - GET /pca9956b/{busId}/{addr}/offset
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET) => Ok("GetOffset"),
            // GetOutputChange - GET /pca9956b/{busId}/{addr}/outputChange
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE) => Ok("GetOutputChange"),
            // GetOverTemp - GET /pca9956b/{busId}/{addr}/overTemp
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OVERTEMP) => Ok("GetOverTemp"),
            // GetPwm - GET /pca9956b/{busId}/{addr}/pwm
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM) => Ok("GetPwm"),
            // GetSleep - GET /pca9956b/{busId}/{addr}/sleep
            &hyper::Method::GET if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP) => Ok("GetSleep"),
            // Reset - POST /pca9956b/{busId}/reset
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_RESET) => Ok("Reset"),
            // SetAddrEnabled - POST /pca9956b/{busId}/{addr}/addr/{num}/enabled/{enabled}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ENABLED_ENABLED) => Ok("SetAddrEnabled"),
            // SetAddrValue - POST /pca9956b/{busId}/{addr}/addr/{num}/addr/{addrVal}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_ADDR_NUM_ADDR_ADDRVAL) => Ok("SetAddrValue"),
            // SetConfig - POST /pca9956b/{busId}/{addr}/config
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CONFIG) => Ok("SetConfig"),
            // SetCurrent - POST /pca9956b/{busId}/{addr}/current/{current}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_CURRENT_CURRENT) => Ok("SetCurrent"),
            // SetFreq - POST /pca9956b/{busId}/{addr}/freq/{freq}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_FREQ_FREQ) => Ok("SetFreq"),
            // SetGroup - POST /pca9956b/{busId}/{addr}/group/{group}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_GROUP_GROUP) => Ok("SetGroup"),
            // SetLedCurrent - POST /pca9956b/{busId}/{addr}/led/{led}/current/{current}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_CURRENT_CURRENT) => Ok("SetLedCurrent"),
            // SetLedError - POST /pca9956b/{busId}/{addr}/led/{led}/error/{error}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_ERROR_ERROR) => Ok("SetLedError"),
            // SetLedInfo - POST /pca9956b/{busId}/{addr}/led/{led}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED) => Ok("SetLedInfo"),
            // SetLedInfoAll - POST /pca9956b/{busId}/{addr}/led
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED) => Ok("SetLedInfoAll"),
            // SetLedPwm - POST /pca9956b/{busId}/{addr}/led/{led}/pwm/{pwm}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_PWM_PWM) => Ok("SetLedPwm"),
            // SetLedState - POST /pca9956b/{busId}/{addr}/led/{led}/state/{state}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_LED_LED_STATE_STATE) => Ok("SetLedState"),
            // SetOffset - POST /pca9956b/{busId}/{addr}/offset/{offset}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OFFSET_OFFSET) => Ok("SetOffset"),
            // SetOutputChange - POST /pca9956b/{busId}/{addr}/outputChange/{outputChange}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_OUTPUTCHANGE_OUTPUTCHANGE) => Ok("SetOutputChange"),
            // SetPwm - POST /pca9956b/{busId}/{addr}/pwm/{pwm}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_PWM_PWM) => Ok("SetPwm"),
            // SetSleep - POST /pca9956b/{busId}/{addr}/sleep/{sleep}
            &hyper::Method::POST if path.matched(paths::ID_PCA9956B_BUSID_ADDR_SLEEP_SLEEP) => Ok("SetSleep"),
            _ => Err(()),
        }
    }
}
