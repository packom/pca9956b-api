use futures;
use futures::{Future, Stream, future, stream};
use hyper;
use hyper::client::HttpConnector;
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, Uri, Response};
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use hyper_openssl::HttpsConnector;
use serde_json;
use std::borrow::Cow;
use std::convert::TryInto;
use std::io::{Read, Error, ErrorKind};
use std::error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use swagger;
use swagger::{ApiError, Connector, client::Service, XSpanIdString, Has, AuthData};
use url::form_urlencoded;
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET};

use crate::models;
use crate::header;

url::define_encode_set! {
    /// This encode set is used for object IDs
    ///
    /// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
    /// the vertical bar (|) is encoded.
    pub ID_ENCODE_SET = [PATH_SEGMENT_ENCODE_SET] | {'|'}
}

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

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(input: &str, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = Uri::from_str(input)?;

    let scheme = uri.scheme_part().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or_else(|| ClientInitError::MissingHost)?;
    let port = uri.port_part().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}{}", scheme, host, port, uri.path().trim_end_matches('/')))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client<F>
{
    /// Inner service
    client_service: Arc<Box<dyn Service<ReqBody=Body, Future=F> + Send + Sync>>,

    /// Base path of the API
    base_path: String,
}

impl<F> fmt::Debug for Client<F>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<F> Clone for Client<F>
{
    fn clone(&self) -> Self {
        Client {
            client_service: self.client_service.clone(),
            base_path: self.base_path.clone(),
        }
    }
}

impl Client<hyper::client::ResponseFuture>
{
    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with `swagger::Connector::builder()`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - Implementation of `hyper::client::Connect` to use for the client
    pub fn try_new_with_connector<C>(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: C,
    ) -> Result<Self, ClientInitError> where
      C: hyper::client::connect::Connect + 'static,
      C::Transport: 'static,
      C::Future: 'static,
    {
        let client_service = Box::new(hyper::client::Client::builder().build(connector));

        Ok(Client {
            client_service: Arc::new(client_service),
            base_path: into_base_path(base_path, protocol)?,
        })
    }

    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    pub fn try_new_http(
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        let http_connector = Connector::builder().build();

        Self::try_new_with_connector(base_path, Some("http"), http_connector)
    }

    /// Create a client with a TLS connection to the server
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    pub fn try_new_https(base_path: &str) -> Result<Self, ClientInitError>
    {
        let https_connector = Connector::builder()
            .https()
            .build()
            .map_err(|e| ClientInitError::SslError(e))?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a TLS connection to the server using a pinned certificate
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_pinned<CA>(
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .build()
            .map_err(|e| ClientInitError::SslError(e))?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_mutual<CA, K, D>(
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: D,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        D: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .client_authentication(client_key, client_certificate)
            .build()
            .map_err(|e| ClientInitError::SslError(e))?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }
}

impl<F> Client<F>
{
    /// Constructor for creating a `Client` by passing in a pre-made `swagger::Service`
    ///
    /// This allows adding custom wrappers around the underlying transport, for example for logging.
    pub fn try_new_with_client_service(
        client_service: Arc<Box<dyn Service<ReqBody=Body, Future=F> + Send + Sync>>,
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        Ok(Client {
            client_service: client_service,
            base_path: into_base_path(base_path, None)?,
        })
    }
}

/// Error type failing to create a Client
#[derive(Debug)]
pub enum ClientInitError {
    /// Invalid URL Scheme
    InvalidScheme,

    /// Invalid URI
    InvalidUri(hyper::http::uri::InvalidUri),

    /// Missing Hostname
    MissingHost,

    /// SSL Connection Error
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
    SslError(native_tls::Error),

    /// SSL Connection Error
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    SslError(openssl::error::ErrorStack),
}

impl From<hyper::http::uri::InvalidUri> for ClientInitError {
    fn from(err: hyper::http::uri::InvalidUri) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &dyn fmt::Debug = self;
        s.fmt(f)
    }
}

impl error::Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}

impl<C, F> Api<C> for Client<F> where
    C: Has<XSpanIdString> ,
    F: Future<Item=Response<Body>, Error=hyper::Error> + Send + 'static
{
    fn clear_error(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=ClearErrorResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/error/clear",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            ClearErrorResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            ClearErrorResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            ClearErrorResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_addr_enabled(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrEnabledResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/addr/{num}/enabled",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrEnabledResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrEnabledResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrEnabledResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_addr_info(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrInfoResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/addr/{num}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::AddrInfo>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrInfoResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrInfoResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrInfoResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_addr_value(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_num: i32,
        context: &C) -> Box<dyn Future<Item=GetAddrValueResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/addr/{num}/addr",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrValueResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrValueResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetAddrValueResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_api(
        &self,
        context: &C) -> Box<dyn Future<Item=GetApiResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/api",
            self.base_path
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 Ok(body.to_string())
                                             )
                                 )
                        .map(move |body| {
                            GetApiResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                404 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 Ok(body.to_string())
                                             )
                                 )
                        .map(move |body| {
                            GetApiResponse::FileNotFound
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_config(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetConfigResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/config",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::Config>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetConfigResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetConfigResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetConfigResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_current(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetCurrentResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/current",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetCurrentResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetCurrentResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetCurrentResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_error(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetErrorResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/error",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetErrorResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetErrorResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetErrorResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_errors(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetErrorsResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/errors",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::LedErrors>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetErrorsResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetErrorsResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetErrorsResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_freq(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetFreqResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/freq",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetFreqResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetFreqResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetFreqResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_group(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetGroupResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/group",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::Group>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetGroupResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetGroupResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetGroupResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_led_current(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedCurrentResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/current",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedCurrentResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedCurrentResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedCurrentResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_led_error(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedErrorResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/error",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::LedError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedErrorResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedErrorResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedErrorResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_led_info(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedInfoResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::LedInfo>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedInfoResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedInfoResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedInfoResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_led_info_all(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetLedInfoAllResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::LedInfoArray>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedInfoAllResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedInfoAllResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedInfoAllResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_led_pwm(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedPwmResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/pwm",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedPwmResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedPwmResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedPwmResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_led_state(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        context: &C) -> Box<dyn Future<Item=GetLedStateResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/state",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::LedState>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedStateResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedStateResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetLedStateResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_offset(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOffsetResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/offset",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOffsetResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOffsetResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOffsetResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_output_change(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOutputChangeResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/outputChange",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OutputChange>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOutputChangeResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOutputChangeResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOutputChangeResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_over_temp(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetOverTempResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/overTemp",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOverTempResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOverTempResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetOverTempResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_pwm(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetPwmResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/pwm",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetPwmResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetPwmResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetPwmResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn get_sleep(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        context: &C) -> Box<dyn Future<Item=GetSleepResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/sleep",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetSleepResponse::OK
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetSleepResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            GetSleepResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn reset(
        &self,
        param_bus_id: i32,
        context: &C) -> Box<dyn Future<Item=ResetResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/reset",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            ResetResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            ResetResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            ResetResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_addr_enabled(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_num: i32,
        param_enabled: bool,
        context: &C) -> Box<dyn Future<Item=SetAddrEnabledResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/addr/{num}/enabled/{enabled}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
            ,enabled=utf8_percent_encode(&param_enabled.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetAddrEnabledResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetAddrEnabledResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetAddrEnabledResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_addr_value(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_num: i32,
        param_addr_val: i32,
        context: &C) -> Box<dyn Future<Item=SetAddrValueResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/addr/{num}/addr/{addr_val}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
            ,addr_val=utf8_percent_encode(&param_addr_val.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetAddrValueResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetAddrValueResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetAddrValueResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_config(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_config: models::Config,
        context: &C) -> Box<dyn Future<Item=SetConfigResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/config",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let body = serde_json::to_string(&param_config).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create header: {} - {}", header, e))))
        });
        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetConfigResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetConfigResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetConfigResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_current(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_current: i32,
        context: &C) -> Box<dyn Future<Item=SetCurrentResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/current/{current}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,current=utf8_percent_encode(&param_current.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetCurrentResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetCurrentResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetCurrentResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_freq(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_freq: i32,
        context: &C) -> Box<dyn Future<Item=SetFreqResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/freq/{freq}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,freq=utf8_percent_encode(&param_freq.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetFreqResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetFreqResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetFreqResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_group(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_group: models::Group,
        context: &C) -> Box<dyn Future<Item=SetGroupResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/group/{group}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,group=utf8_percent_encode(&param_group.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetGroupResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetGroupResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetGroupResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_led_current(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        param_current: i32,
        context: &C) -> Box<dyn Future<Item=SetLedCurrentResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/current/{current}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
            ,current=utf8_percent_encode(&param_current.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetLedCurrentResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedCurrentResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedCurrentResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_led_error(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        param_error: models::LedError,
        context: &C) -> Box<dyn Future<Item=SetLedErrorResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/error/{error}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
            ,error=utf8_percent_encode(&param_error.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetLedErrorResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedErrorResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedErrorResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_led_info(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        param_led_info: models::LedInfo,
        context: &C) -> Box<dyn Future<Item=SetLedInfoResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let body = serde_json::to_string(&param_led_info).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create header: {} - {}", header, e))))
        });
        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetLedInfoResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedInfoResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedInfoResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_led_info_all(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led_info_array: models::LedInfoArray,
        context: &C) -> Box<dyn Future<Item=SetLedInfoAllResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let body = serde_json::to_string(&param_led_info_array).expect("impossible to fail to serialize");
                *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create header: {} - {}", header, e))))
        });
        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetLedInfoAllResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedInfoAllResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedInfoAllResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_led_pwm(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        param_pwm: i32,
        context: &C) -> Box<dyn Future<Item=SetLedPwmResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/pwm/{pwm}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
            ,pwm=utf8_percent_encode(&param_pwm.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetLedPwmResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedPwmResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedPwmResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_led_state(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_led: i32,
        param_state: models::LedState,
        context: &C) -> Box<dyn Future<Item=SetLedStateResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/led/{led}/state/{state}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
            ,state=utf8_percent_encode(&param_state.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetLedStateResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedStateResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetLedStateResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_offset(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_offset: i32,
        context: &C) -> Box<dyn Future<Item=SetOffsetResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/offset/{offset}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,offset=utf8_percent_encode(&param_offset.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetOffsetResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetOffsetResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetOffsetResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_output_change(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_output_change: models::OutputChange,
        context: &C) -> Box<dyn Future<Item=SetOutputChangeResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/outputChange/{output_change}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,output_change=utf8_percent_encode(&param_output_change.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetOutputChangeResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetOutputChangeResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetOutputChangeResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_pwm(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_pwm: i32,
        context: &C) -> Box<dyn Future<Item=SetPwmResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/pwm/{pwm}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,pwm=utf8_percent_encode(&param_pwm.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetPwmResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetPwmResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetPwmResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

    fn set_sleep(
        &self,
        param_bus_id: i32,
        param_addr: i32,
        param_sleep: bool,
        context: &C) -> Box<dyn Future<Item=SetSleepResponse, Error=ApiError> + Send>
    {
        let mut uri = format!(
            "{}/pca9956b/{bus_id}/{addr}/sleep/{sleep}",
            self.base_path
            ,bus_id=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
            ,addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
            ,sleep=utf8_percent_encode(&param_sleep.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(ApiError(format!("Unable to build URI: {}", err)))),
        };

        let mut request = match hyper::Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Box::new(future::err(ApiError(format!("Unable to create request: {}", e))))
        };

        let header = HeaderValue::from_str((context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Box::new(future::err(ApiError(format!("Unable to create X-Span ID header value: {}", e))))
        });

        Box::new(self.client_service.request(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.into_body();
                    Box::new(
                        future::ok(
                            SetSleepResponse::OK
                        )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                400 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetSleepResponse::BadRequest
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                502 => {
                    let body = response.into_body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body|
                        str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|
                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())
                                             )
                                 )
                        .map(move |body| {
                            SetSleepResponse::OperationFailed
                            (body)
                        })
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.into_body()
                            .take(100)
                            .concat2()
                            .then(move |body|
                                future::err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                    code,
                                    headers,
                                    match body {
                                        Ok(ref body) => match str::from_utf8(body) {
                                            Ok(body) => Cow::from(body),
                                            Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                        },
                                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                    })))
                            )
                    ) as Box<dyn Future<Item=_, Error=_> + Send>
                }
            }
        }))
    }

}
