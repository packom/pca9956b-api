#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


/// address value
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Addr(i32);

impl ::std::convert::From<i32> for Addr {
    fn from(x: i32) -> Self {
        Addr(x)
    }
}

impl ::std::convert::From<Addr> for i32 {
    fn from(x: Addr) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Addr {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Addr {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// whether this address is enabled
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct AddrEnabled(bool);

impl ::std::convert::From<bool> for AddrEnabled {
    fn from(x: bool) -> Self {
        AddrEnabled(x)
    }
}

impl ::std::convert::From<AddrEnabled> for bool {
    fn from(x: AddrEnabled) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for AddrEnabled {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl ::std::ops::DerefMut for AddrEnabled {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}


/// address identifier 0=all call, 1-3=sub addrs
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct AddrIndex(i32);

impl ::std::convert::From<i32> for AddrIndex {
    fn from(x: i32) -> Self {
        AddrIndex(x)
    }
}

impl ::std::convert::From<AddrIndex> for i32 {
    fn from(x: AddrIndex) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for AddrIndex {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for AddrIndex {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Address information (used to configure sub/all call addresses)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// Some error text
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct ApiError(String);

impl ::std::convert::From<String> for ApiError {
    fn from(x: String) -> Self {
        ApiError(x)
    }
}

impl ::std::convert::From<ApiError> for String {
    fn from(x: ApiError) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for ApiError {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for ApiError {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Client sent a bad request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// A valid bus ID as returned by i2c_bus_list
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct BusId(i32);

impl ::std::convert::From<i32> for BusId {
    fn from(x: i32) -> Self {
        BusId(x)
    }
}

impl ::std::convert::From<BusId> for i32 {
    fn from(x: BusId) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for BusId {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for BusId {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// PCA9956B IC configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// current value
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Current(i32);

impl ::std::convert::From<i32> for Current {
    fn from(x: i32) -> Self {
        Current(x)
    }
}

impl ::std::convert::From<Current> for i32 {
    fn from(x: Current) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Current {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Current {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Whether there are any LED errors
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Error(bool);

impl ::std::convert::From<bool> for Error {
    fn from(x: bool) -> Self {
        Error(x)
    }
}

impl ::std::convert::From<Error> for bool {
    fn from(x: Error) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Error {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl ::std::ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}


/// frequency multipier, 0-255 inclusive
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Freq(i32);

impl ::std::convert::From<i32> for Freq {
    fn from(x: i32) -> Self {
        Freq(x)
    }
}

impl ::std::convert::From<Freq> for i32 {
    fn from(x: Freq) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Freq {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Freq {
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum Group { 
    #[serde(rename = "dim")]
    DIM,
    #[serde(rename = "blink")]
    BLINK,
}

impl ::std::fmt::Display for Group {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            Group::DIM => write!(f, "{}", "dim"),
            Group::BLINK => write!(f, "{}", "blink"),
        }
    }
}

impl ::std::str::FromStr for Group {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dim" => Ok(Group::DIM),
            "blink" => Ok(Group::BLINK),
            e => Err(format!("Invalid value {}", e)),
        }
    }
}

/// LED error
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
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

impl ::std::fmt::Display for LedError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            LedError::NONE => write!(f, "{}", "none"),
            LedError::SHORT => write!(f, "{}", "short"),
            LedError::OPEN => write!(f, "{}", "open"),
            LedError::DNE => write!(f, "{}", "dne"),
        }
    }
}

impl ::std::str::FromStr for LedError {
    type Err = (String);
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(LedError::NONE),
            "short" => Ok(LedError::SHORT),
            "open" => Ok(LedError::OPEN),
            "dne" => Ok(LedError::DNE),
            e => Err(format!("Invalid value {}", e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedErrors(Vec<AddrInfo>);

impl ::std::convert::From<Vec<AddrInfo>> for LedErrors {
    fn from(x: Vec<AddrInfo>) -> Self {
        LedErrors(x)
    }
}

impl ::std::convert::From<LedErrors> for Vec<AddrInfo> {
    fn from(x: LedErrors) -> Self {
        x.0
    }
}

impl ::std::iter::FromIterator<AddrInfo> for LedErrors {
    fn from_iter<U: IntoIterator<Item=AddrInfo>>(u: U) -> Self {
        LedErrors(Vec::<AddrInfo>::from_iter(u))
    }
}

impl ::std::iter::IntoIterator for LedErrors {
    type Item = AddrInfo;
    type IntoIter = ::std::vec::IntoIter<AddrInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a LedErrors {
    type Item = &'a AddrInfo;
    type IntoIter = ::std::slice::Iter<'a, AddrInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a mut LedErrors {
    type Item = &'a mut AddrInfo;
    type IntoIter = ::std::slice::IterMut<'a, AddrInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl ::std::ops::Deref for LedErrors {
    type Target = Vec<AddrInfo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for LedErrors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// LED number
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct LedIndex(i32);

impl ::std::convert::From<i32> for LedIndex {
    fn from(x: i32) -> Self {
        LedIndex(x)
    }
}

impl ::std::convert::From<LedIndex> for i32 {
    fn from(x: LedIndex) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for LedIndex {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for LedIndex {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Information about a specific LED
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedInfoArray(Vec<LedInfo>);

impl ::std::convert::From<Vec<LedInfo>> for LedInfoArray {
    fn from(x: Vec<LedInfo>) -> Self {
        LedInfoArray(x)
    }
}

impl ::std::convert::From<LedInfoArray> for Vec<LedInfo> {
    fn from(x: LedInfoArray) -> Self {
        x.0
    }
}

impl ::std::iter::FromIterator<LedInfo> for LedInfoArray {
    fn from_iter<U: IntoIterator<Item=LedInfo>>(u: U) -> Self {
        LedInfoArray(Vec::<LedInfo>::from_iter(u))
    }
}

impl ::std::iter::IntoIterator for LedInfoArray {
    type Item = LedInfo;
    type IntoIter = ::std::vec::IntoIter<LedInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a LedInfoArray {
    type Item = &'a LedInfo;
    type IntoIter = ::std::slice::Iter<'a, LedInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a mut LedInfoArray {
    type Item = &'a mut LedInfo;
    type IntoIter = ::std::slice::IterMut<'a, LedInfo>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl ::std::ops::Deref for LedInfoArray {
    type Target = Vec<LedInfo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for LedInfoArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// LED state
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
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

impl ::std::fmt::Display for LedState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            LedState::FALSE => write!(f, "{}", "false"),
            LedState::TRUE => write!(f, "{}", "true"),
            LedState::PWM => write!(f, "{}", "pwm"),
            LedState::PWMPLUS => write!(f, "{}", "pwmPlus"),
        }
    }
}

impl ::std::str::FromStr for LedState {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "false" => Ok(LedState::FALSE),
            "true" => Ok(LedState::TRUE),
            "pwm" => Ok(LedState::PWM),
            "pwmPlus" => Ok(LedState::PWMPLUS),
            e => Err(format!("Invalid value {}", e)),
        }
    }
}

/// offset value, number of (125ns) clock cycles between on times of consecutive LEDs
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Offset(i32);

impl ::std::convert::From<i32> for Offset {
    fn from(x: i32) -> Self {
        Offset(x)
    }
}

impl ::std::convert::From<Offset> for i32 {
    fn from(x: Offset) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Offset {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Offset {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Operation as a whole failed.  It is possible that part of the operation succeeded so the device may be left in a bad state.  A reset may be in order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// whether output changes on I2C STOP or ACK
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum OutputChange { 
    #[serde(rename = "stop")]
    STOP,
    #[serde(rename = "ack")]
    ACK,
}

impl ::std::fmt::Display for OutputChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            OutputChange::STOP => write!(f, "{}", "stop"),
            OutputChange::ACK => write!(f, "{}", "ack"),
        }
    }
}

impl ::std::str::FromStr for OutputChange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stop" => Ok(OutputChange::STOP),
            "ack" => Ok(OutputChange::ACK),
            e => Err(format!("Invalid value {}", e)),
        }
    }
}

/// Whether there is an over-temperature error
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct OverTemp(bool);

impl ::std::convert::From<bool> for OverTemp {
    fn from(x: bool) -> Self {
        OverTemp(x)
    }
}

impl ::std::convert::From<OverTemp> for bool {
    fn from(x: OverTemp) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for OverTemp {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl ::std::ops::DerefMut for OverTemp {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}


/// PWM value, 0-255 inclusive
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Pwm(i32);

impl ::std::convert::From<i32> for Pwm {
    fn from(x: i32) -> Self {
        Pwm(x)
    }
}

impl ::std::convert::From<Pwm> for i32 {
    fn from(x: Pwm) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Pwm {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Pwm {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// whether sleep is enabled (oscillator not running when sleep is enabled, and it takes >= 500us to bring the device out of sleep)
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Sleep(bool);

impl ::std::convert::From<bool> for Sleep {
    fn from(x: bool) -> Self {
        Sleep(x)
    }
}

impl ::std::convert::From<Sleep> for bool {
    fn from(x: Sleep) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Sleep {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl ::std::ops::DerefMut for Sleep {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}


/// A YAML file
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Yaml(String);

impl ::std::convert::From<String> for Yaml {
    fn from(x: String) -> Self {
        Yaml(x)
    }
}

impl ::std::convert::From<Yaml> for String {
    fn from(x: Yaml) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Yaml {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for Yaml {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

// Added manually

use std::str::FromStr;
use std::num::ParseIntError;
use std::str::ParseBoolError;

impl FromStr for Addr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Addr(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for AddrEnabled {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<bool>() {
            Ok(val) => Ok(AddrEnabled(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for AddrIndex {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(AddrIndex(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for Current {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Current(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for Freq {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Freq(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for LedIndex {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(LedIndex(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for Offset {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Offset(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for Pwm {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(val) => Ok(Pwm(val)),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for Sleep {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<bool>() {
            Ok(val) => Ok(Sleep(val)),
            Err(e) => Err(e)
        }
    }
}
