#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;


/// address value
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Addr(i32);

impl std::convert::From<i32> for Addr {
    fn from(x: i32) -> Self {
        Addr(x)
    }
}


impl std::convert::From<Addr> for i32 {
    fn from(x: Addr) -> Self {
        x.0
    }
}

impl std::ops::Deref for Addr {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Addr {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// whether this address is enabled
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddrEnabled(bool);

impl std::convert::From<bool> for AddrEnabled {
    fn from(x: bool) -> Self {
        AddrEnabled(x)
    }
}


impl std::convert::From<AddrEnabled> for bool {
    fn from(x: AddrEnabled) -> Self {
        x.0
    }
}

impl std::ops::Deref for AddrEnabled {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for AddrEnabled {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}



/// address identifier 0=all call, 1-3=sub addrs
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddrIndex(i32);

impl std::convert::From<i32> for AddrIndex {
    fn from(x: i32) -> Self {
        AddrIndex(x)
    }
}


impl std::convert::From<AddrIndex> for i32 {
    fn from(x: AddrIndex) -> Self {
        x.0
    }
}

impl std::ops::Deref for AddrIndex {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for AddrIndex {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// Address information (used to configure sub/all call addresses)
// Methods for converting between header::IntoHeaderValue<AddrInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddrInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddrInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddrInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddrInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddrInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddrInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddrInfo {
    /// address identifier 0=all call, 1-3=sub addrs
    #[serde(rename = "index")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index: Option<u32>,

    /// whether this address is enabled
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,

    /// address value
    #[serde(rename = "addr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub addr: Option<u32>,

}

impl AddrInfo {
    pub fn new() -> AddrInfo {
        AddrInfo {
            index: None,
            enabled: None,
            addr: None,
        }
    }
}

/// Converts the AddrInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddrInfo {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref index) = self.index {
            params.push("index".to_string());
            params.push(index.to_string());
        }


        if let Some(ref enabled) = self.enabled {
            params.push("enabled".to_string());
            params.push(enabled.to_string());
        }


        if let Some(ref addr) = self.addr {
            params.push("addr".to_string());
            params.push(addr.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddrInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddrInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub index: Vec<u32>,
            pub enabled: Vec<bool>,
            pub addr: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddrInfo".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "index" => intermediate_rep.index.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "enabled" => intermediate_rep.enabled.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "addr" => intermediate_rep.addr.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddrInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddrInfo {
            index: intermediate_rep.index.into_iter().next(),
            enabled: intermediate_rep.enabled.into_iter().next(),
            addr: intermediate_rep.addr.into_iter().next(),
        })
    }
}



/// Some error text
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiError(String);

impl std::convert::From<String> for ApiError {
    fn from(x: String) -> Self {
        ApiError(x)
    }
}

impl std::string::ToString for ApiError {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for ApiError {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(ApiError(x.to_string()))
    }
}

impl std::convert::From<ApiError> for String {
    fn from(x: ApiError) -> Self {
        x.0
    }
}

impl std::ops::Deref for ApiError {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for ApiError {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// Client sent a bad request
// Methods for converting between header::IntoHeaderValue<BadRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BadRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BadRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BadRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BadRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BadRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BadRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BadRequest {
    /// what parameter was at fault
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameter: Option<String>,

    /// Error description
    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,

}

impl BadRequest {
    pub fn new() -> BadRequest {
        BadRequest {
            parameter: None,
            error: None,
        }
    }
}

/// Converts the BadRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BadRequest {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref parameter) = self.parameter {
            params.push("parameter".to_string());
            params.push(parameter.to_string());
        }


        if let Some(ref error) = self.error {
            params.push("error".to_string());
            params.push(error.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BadRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BadRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub parameter: Vec<String>,
            pub error: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BadRequest".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "parameter" => intermediate_rep.parameter.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => intermediate_rep.error.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BadRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BadRequest {
            parameter: intermediate_rep.parameter.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
        })
    }
}



/// A valid bus ID as returned by i2c_bus_list
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BusId(i32);

impl std::convert::From<i32> for BusId {
    fn from(x: i32) -> Self {
        BusId(x)
    }
}


impl std::convert::From<BusId> for i32 {
    fn from(x: BusId) -> Self {
        x.0
    }
}

impl std::ops::Deref for BusId {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for BusId {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// PCA9956B IC configuration
// Methods for converting between header::IntoHeaderValue<Config> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Config>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Config>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Config - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Config> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Config as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Config - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Config {
    /// whether sleep is enabled (oscillator not running when sleep is enabled, and it takes >= 500us to bring the device out of sleep)
    #[serde(rename = "sleep")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sleep: Option<bool>,

    #[serde(rename = "group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<models::Group>,

    #[serde(rename = "outputChange")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_change: Option<models::OutputChange>,

    /// PWM value, 0-255 inclusive
    #[serde(rename = "pwm")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pwm: Option<u32>,

    /// frequency multipier, 0-255 inclusive
    #[serde(rename = "freq")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub freq: Option<u32>,

    /// offset value, number of (125ns) clock cycles between on times of consecutive LEDs
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset: Option<u32>,

    /// current value
    #[serde(rename = "current")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current: Option<u32>,

    #[serde(rename = "addr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub addr: Option<Vec<models::AddrInfo>>,

}

impl Config {
    pub fn new() -> Config {
        Config {
            sleep: None,
            group: None,
            output_change: None,
            pwm: None,
            freq: None,
            offset: None,
            current: None,
            addr: None,
        }
    }
}

/// Converts the Config value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Config {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref sleep) = self.sleep {
            params.push("sleep".to_string());
            params.push(sleep.to_string());
        }

        // Skipping group in query parameter serialization

        // Skipping outputChange in query parameter serialization


        if let Some(ref pwm) = self.pwm {
            params.push("pwm".to_string());
            params.push(pwm.to_string());
        }


        if let Some(ref freq) = self.freq {
            params.push("freq".to_string());
            params.push(freq.to_string());
        }


        if let Some(ref offset) = self.offset {
            params.push("offset".to_string());
            params.push(offset.to_string());
        }


        if let Some(ref current) = self.current {
            params.push("current".to_string());
            params.push(current.to_string());
        }

        // Skipping addr in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Config value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Config {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub sleep: Vec<bool>,
            pub group: Vec<models::Group>,
            pub output_change: Vec<models::OutputChange>,
            pub pwm: Vec<u32>,
            pub freq: Vec<u32>,
            pub offset: Vec<u32>,
            pub current: Vec<u32>,
            pub addr: Vec<Vec<models::AddrInfo>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Config".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "sleep" => intermediate_rep.sleep.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "group" => intermediate_rep.group.push(models::Group::from_str(val).map_err(|x| format!("{}", x))?),
                    "outputChange" => intermediate_rep.output_change.push(models::OutputChange::from_str(val).map_err(|x| format!("{}", x))?),
                    "pwm" => intermediate_rep.pwm.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "freq" => intermediate_rep.freq.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "offset" => intermediate_rep.offset.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "current" => intermediate_rep.current.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "addr" => return std::result::Result::Err("Parsing a container in this style is not supported in Config".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Config".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Config {
            sleep: intermediate_rep.sleep.into_iter().next(),
            group: intermediate_rep.group.into_iter().next(),
            output_change: intermediate_rep.output_change.into_iter().next(),
            pwm: intermediate_rep.pwm.into_iter().next(),
            freq: intermediate_rep.freq.into_iter().next(),
            offset: intermediate_rep.offset.into_iter().next(),
            current: intermediate_rep.current.into_iter().next(),
            addr: intermediate_rep.addr.into_iter().next(),
        })
    }
}



/// current value
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Current(i32);

impl std::convert::From<i32> for Current {
    fn from(x: i32) -> Self {
        Current(x)
    }
}


impl std::convert::From<Current> for i32 {
    fn from(x: Current) -> Self {
        x.0
    }
}

impl std::ops::Deref for Current {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Current {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// Whether there are any LED errors
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error(bool);

impl std::convert::From<bool> for Error {
    fn from(x: bool) -> Self {
        Error(x)
    }
}


impl std::convert::From<Error> for bool {
    fn from(x: Error) -> Self {
        x.0
    }
}

impl std::ops::Deref for Error {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}



/// frequency multipier, 0-255 inclusive
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Freq(i32);

impl std::convert::From<i32> for Freq {
    fn from(x: i32) -> Self {
        Freq(x)
    }
}


impl std::convert::From<Freq> for i32 {
    fn from(x: Freq) -> Self {
        x.0
    }
}

impl std::ops::Deref for Freq {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Freq {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// whether config pwm/freq values control dimming or blinking
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum Group { 
    #[serde(rename = "dim")]
    DIM,
    #[serde(rename = "blink")]
    BLINK,
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            Group::DIM => write!(f, "{}", "dim"),
            Group::BLINK => write!(f, "{}", "blink"),
        }
    }
}

impl std::str::FromStr for Group {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "dim" => std::result::Result::Ok(Group::DIM),
            "blink" => std::result::Result::Ok(Group::BLINK),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


/// LED error
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum LedError { 
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "short")]
    SHORT,
    #[serde(rename = "open")]
    OPEN,
    #[serde(rename = "dne")]
    DNE,
}

impl std::fmt::Display for LedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            LedError::NONE => write!(f, "{}", "none"),
            LedError::SHORT => write!(f, "{}", "short"),
            LedError::OPEN => write!(f, "{}", "open"),
            LedError::DNE => write!(f, "{}", "dne"),
        }
    }
}

impl std::str::FromStr for LedError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "none" => std::result::Result::Ok(LedError::NONE),
            "short" => std::result::Result::Ok(LedError::SHORT),
            "open" => std::result::Result::Ok(LedError::OPEN),
            "dne" => std::result::Result::Ok(LedError::DNE),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<LedErrors> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LedErrors>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LedErrors>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LedErrors - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LedErrors> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LedErrors as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LedErrors - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LedErrors(
    Vec<AddrInfo>
);

impl std::convert::From<Vec<AddrInfo>> for LedErrors {
    fn from(x: Vec<AddrInfo>) -> Self {
        LedErrors(x)
    }
}

impl std::convert::From<LedErrors> for Vec<AddrInfo> {
    fn from(x: LedErrors) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<AddrInfo> for LedErrors {
    fn from_iter<U: IntoIterator<Item=AddrInfo>>(u: U) -> Self {
        LedErrors(Vec::<AddrInfo>::from_iter(u))
    }
}

impl std::iter::IntoIterator for LedErrors {
    type Item = AddrInfo;
    type IntoIter = std::vec::IntoIter<AddrInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a LedErrors {
    type Item = &'a AddrInfo;
    type IntoIter = std::slice::Iter<'a, AddrInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut LedErrors {
    type Item = &'a mut AddrInfo;
    type IntoIter = std::slice::IterMut<'a, AddrInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for LedErrors {
    type Target = Vec<AddrInfo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LedErrors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the LedErrors value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LedErrors {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LedErrors value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LedErrors {
    type Err = <AddrInfo as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(LedErrors(items))
    }
}



/// LED number
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LedIndex(i32);

impl std::convert::From<i32> for LedIndex {
    fn from(x: i32) -> Self {
        LedIndex(x)
    }
}


impl std::convert::From<LedIndex> for i32 {
    fn from(x: LedIndex) -> Self {
        x.0
    }
}

impl std::ops::Deref for LedIndex {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for LedIndex {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// Information about a specific LED
// Methods for converting between header::IntoHeaderValue<LedInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LedInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LedInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LedInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LedInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LedInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LedInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LedInfo {
    /// LED number
    #[serde(rename = "index")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index: Option<u32>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<models::LedState>,

    /// PWM value, 0-255 inclusive
    #[serde(rename = "pwm")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pwm: Option<u32>,

    /// current value
    #[serde(rename = "current")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current: Option<u32>,

    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<models::LedError>,

}

impl LedInfo {
    pub fn new() -> LedInfo {
        LedInfo {
            index: None,
            state: None,
            pwm: None,
            current: None,
            error: None,
        }
    }
}

/// Converts the LedInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LedInfo {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref index) = self.index {
            params.push("index".to_string());
            params.push(index.to_string());
        }

        // Skipping state in query parameter serialization


        if let Some(ref pwm) = self.pwm {
            params.push("pwm".to_string());
            params.push(pwm.to_string());
        }


        if let Some(ref current) = self.current {
            params.push("current".to_string());
            params.push(current.to_string());
        }

        // Skipping error in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LedInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LedInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub index: Vec<u32>,
            pub state: Vec<models::LedState>,
            pub pwm: Vec<u32>,
            pub current: Vec<u32>,
            pub error: Vec<models::LedError>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LedInfo".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "index" => intermediate_rep.index.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "state" => intermediate_rep.state.push(models::LedState::from_str(val).map_err(|x| format!("{}", x))?),
                    "pwm" => intermediate_rep.pwm.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "current" => intermediate_rep.current.push(u32::from_str(val).map_err(|x| format!("{}", x))?),
                    "error" => intermediate_rep.error.push(models::LedError::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LedInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LedInfo {
            index: intermediate_rep.index.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            pwm: intermediate_rep.pwm.into_iter().next(),
            current: intermediate_rep.current.into_iter().next(),
            error: intermediate_rep.error.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<LedInfoArray> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LedInfoArray>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LedInfoArray>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LedInfoArray - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LedInfoArray> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LedInfoArray as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LedInfoArray - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LedInfoArray(
    Vec<LedInfo>
);

impl std::convert::From<Vec<LedInfo>> for LedInfoArray {
    fn from(x: Vec<LedInfo>) -> Self {
        LedInfoArray(x)
    }
}

impl std::convert::From<LedInfoArray> for Vec<LedInfo> {
    fn from(x: LedInfoArray) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<LedInfo> for LedInfoArray {
    fn from_iter<U: IntoIterator<Item=LedInfo>>(u: U) -> Self {
        LedInfoArray(Vec::<LedInfo>::from_iter(u))
    }
}

impl std::iter::IntoIterator for LedInfoArray {
    type Item = LedInfo;
    type IntoIter = std::vec::IntoIter<LedInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a LedInfoArray {
    type Item = &'a LedInfo;
    type IntoIter = std::slice::Iter<'a, LedInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut LedInfoArray {
    type Item = &'a mut LedInfo;
    type IntoIter = std::slice::IterMut<'a, LedInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl std::ops::Deref for LedInfoArray {
    type Target = Vec<LedInfo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LedInfoArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the LedInfoArray value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LedInfoArray {
    fn to_string(&self) -> String {
        self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LedInfoArray value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LedInfoArray {
    type Err = <LedInfo as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(LedInfoArray(items))
    }
}



/// LED state
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum LedState { 
    #[serde(rename = "false")]
    FALSE,
    #[serde(rename = "true")]
    TRUE,
    #[serde(rename = "pwm")]
    PWM,
    #[serde(rename = "pwmPlus")]
    PWMPLUS,
}

impl std::fmt::Display for LedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            LedState::FALSE => write!(f, "{}", "false"),
            LedState::TRUE => write!(f, "{}", "true"),
            LedState::PWM => write!(f, "{}", "pwm"),
            LedState::PWMPLUS => write!(f, "{}", "pwmPlus"),
        }
    }
}

impl std::str::FromStr for LedState {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "false" => std::result::Result::Ok(LedState::FALSE),
            "true" => std::result::Result::Ok(LedState::TRUE),
            "pwm" => std::result::Result::Ok(LedState::PWM),
            "pwmPlus" => std::result::Result::Ok(LedState::PWMPLUS),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


/// offset value, number of (125ns) clock cycles between on times of consecutive LEDs
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Offset(i32);

impl std::convert::From<i32> for Offset {
    fn from(x: i32) -> Self {
        Offset(x)
    }
}


impl std::convert::From<Offset> for i32 {
    fn from(x: Offset) -> Self {
        x.0
    }
}

impl std::ops::Deref for Offset {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Offset {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// Operation as a whole failed.  It is possible that part of the operation succeeded so the device may be left in a bad state.  A reset may be in order
// Methods for converting between header::IntoHeaderValue<OpError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OpError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OpError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OpError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OpError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OpError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OpError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OpError {
    /// Error description
    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,

}

impl OpError {
    pub fn new() -> OpError {
        OpError {
            error: None,
        }
    }
}

/// Converts the OpError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OpError {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref error) = self.error {
            params.push("error".to_string());
            params.push(error.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OpError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OpError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub error: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OpError".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "error" => intermediate_rep.error.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing OpError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OpError {
            error: intermediate_rep.error.into_iter().next(),
        })
    }
}



/// whether output changes on I2C STOP or ACK
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum OutputChange { 
    #[serde(rename = "stop")]
    STOP,
    #[serde(rename = "ack")]
    ACK,
}

impl std::fmt::Display for OutputChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            OutputChange::STOP => write!(f, "{}", "stop"),
            OutputChange::ACK => write!(f, "{}", "ack"),
        }
    }
}

impl std::str::FromStr for OutputChange {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "stop" => std::result::Result::Ok(OutputChange::STOP),
            "ack" => std::result::Result::Ok(OutputChange::ACK),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


/// Whether there is an over-temperature error
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OverTemp(bool);

impl std::convert::From<bool> for OverTemp {
    fn from(x: bool) -> Self {
        OverTemp(x)
    }
}


impl std::convert::From<OverTemp> for bool {
    fn from(x: OverTemp) -> Self {
        x.0
    }
}

impl std::ops::Deref for OverTemp {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for OverTemp {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}



/// PWM value, 0-255 inclusive
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Pwm(i32);

impl std::convert::From<i32> for Pwm {
    fn from(x: i32) -> Self {
        Pwm(x)
    }
}


impl std::convert::From<Pwm> for i32 {
    fn from(x: Pwm) -> Self {
        x.0
    }
}

impl std::ops::Deref for Pwm {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for Pwm {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}



/// whether sleep is enabled (oscillator not running when sleep is enabled, and it takes >= 500us to bring the device out of sleep)
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Sleep(bool);

impl std::convert::From<bool> for Sleep {
    fn from(x: bool) -> Self {
        Sleep(x)
    }
}


impl std::convert::From<Sleep> for bool {
    fn from(x: Sleep) -> Self {
        x.0
    }
}

impl std::ops::Deref for Sleep {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for Sleep {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}



/// A YAML file
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Yaml(String);

impl std::convert::From<String> for Yaml {
    fn from(x: String) -> Self {
        Yaml(x)
    }
}

impl std::string::ToString for Yaml {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Yaml {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Yaml(x.to_string()))
    }
}

impl std::convert::From<Yaml> for String {
    fn from(x: Yaml) -> Self {
        x.0
    }
}

impl std::ops::Deref for Yaml {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Yaml {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


