#![allow(unused_extern_crates)]
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate chrono;
extern crate url;



use hyper;
use hyper::header::{Headers, ContentType};
use hyper::Uri;
use self::url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET};
use futures;
use futures::{Future, Stream};
use futures::{future, stream};
use self::tokio_core::reactor::Handle;
use std::borrow::Cow;
use std::io::{Read, Error, ErrorKind};
use std::error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::str;
use std::str::FromStr;

use mimetypes;

use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;

use swagger::{ApiError, XSpanId, XSpanIdString, Has, AuthData};

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
use models;

define_encode_set! {
    /// This encode set is used for object IDs
    ///
    /// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
    /// the vertical bar (|) is encoded.
    pub ID_ENCODE_SET = [PATH_SEGMENT_ENCODE_SET] | {'|'}
}

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(input: &str, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = Uri::from_str(input)?;

    let scheme = uri.scheme().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or_else(|| ClientInitError::MissingHost)?;
    let port = uri.port().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}", scheme, host, port))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client<F> where
  F: Future<Item=hyper::Response, Error=hyper::Error> + 'static {
    client_service: Arc<Box<hyper::client::Service<Request=hyper::Request<hyper::Body>, Response=hyper::Response, Error=hyper::Error, Future=F>>>,
    base_path: String,
}

impl<F> fmt::Debug for Client<F> where
   F: Future<Item=hyper::Response, Error=hyper::Error>  + 'static {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<F> Clone for Client<F> where
   F: Future<Item=hyper::Response, Error=hyper::Error>  + 'static {
    fn clone(&self) -> Self {
        Client {
            client_service: self.client_service.clone(),
            base_path: self.base_path.clone()
        }
    }
}

impl Client<hyper::client::FutureResponse> {

    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    pub fn try_new_http(handle: Handle, base_path: &str) -> Result<Client<hyper::client::FutureResponse>, ClientInitError> {
        let http_connector = swagger::http_connector();
        Self::try_new_with_connector::<hyper::client::HttpConnector>(
            handle,
            base_path,
            Some("http"),
            http_connector,
        )
    }

    /// Create a client with a TLS connection to the server.
    ///
    /// # Arguments
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    pub fn try_new_https<CA>(
        handle: Handle,
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Client<hyper::client::FutureResponse>, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = swagger::https_connector(ca_certificate);
        Self::try_new_with_connector::<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>(
            handle,
            base_path,
            Some("https"),
            https_connector,
        )
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    pub fn try_new_https_mutual<CA, K, C>(
        handle: Handle,
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: C,
    ) -> Result<Client<hyper::client::FutureResponse>, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        C: AsRef<Path>,
    {
        let https_connector =
            swagger::https_mutual_connector(ca_certificate, client_key, client_certificate);
        Self::try_new_with_connector::<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>(
            handle,
            base_path,
            Some("https"),
            https_connector,
        )
    }

    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with
    /// `swagger::{http_connector, https_connector, https_mutual_connector}`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `handle` - tokio reactor handle to use for execution
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector_fn` - Function which returns an implementation of `hyper::client::Connect`
    pub fn try_new_with_connector<C>(
        handle: Handle,
        base_path: &str,
        protocol: Option<&'static str>,
        connector_fn: Box<Fn(&Handle) -> C + Send + Sync>,
    ) -> Result<Client<hyper::client::FutureResponse>, ClientInitError>
    where
        C: hyper::client::Connect + hyper::client::Service,
    {
        let connector = connector_fn(&handle);
        let client_service = Box::new(hyper::Client::configure().connector(connector).build(
            &handle,
        ));

        Ok(Client {
            client_service: Arc::new(client_service),
            base_path: into_base_path(base_path, protocol)?,
        })
    }

    /// Constructor for creating a `Client` by passing in a pre-made `hyper` client.
    ///
    /// One should avoid relying on this function if possible, since it adds a dependency on the underlying transport
    /// implementation, which it would be better to abstract away. Therefore, using this function may lead to a loss of
    /// code generality, which may make it harder to move the application to a serverless environment, for example.
    ///
    /// The reason for this function's existence is to support legacy test code, which did mocking at the hyper layer.
    /// This is not a recommended way to write new tests. If other reasons are found for using this function, they
    /// should be mentioned here.
    #[deprecated(note="Use try_new_with_client_service instead")]
    pub fn try_new_with_hyper_client(
        hyper_client: Arc<Box<hyper::client::Service<Request=hyper::Request<hyper::Body>, Response=hyper::Response, Error=hyper::Error, Future=hyper::client::FutureResponse>>>,
        handle: Handle,
        base_path: &str
    ) -> Result<Client<hyper::client::FutureResponse>, ClientInitError>
    {
        Ok(Client {
            client_service: hyper_client,
            base_path: into_base_path(base_path, None)?,
        })
    }
}

impl<F> Client<F> where
    F: Future<Item=hyper::Response, Error=hyper::Error>  + 'static
{
    /// Constructor for creating a `Client` by passing in a pre-made `hyper` client Service.
    ///
    /// This allows adding custom wrappers around the underlying transport, for example for logging.
    pub fn try_new_with_client_service(client_service: Arc<Box<hyper::client::Service<Request=hyper::Request<hyper::Body>, Response=hyper::Response, Error=hyper::Error, Future=F>>>,
                                       handle: Handle,
                                       base_path: &str)
                                    -> Result<Client<F>, ClientInitError>
    {
        Ok(Client {
            client_service: client_service,
            base_path: into_base_path(base_path, None)?,
        })
    }
}

impl<F, C> Api<C> for Client<F> where
    F: Future<Item=hyper::Response, Error=hyper::Error>  + 'static,
    C: Has<XSpanIdString> {

    fn clear_error(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=ClearErrorResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/error/clear",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            ClearErrorResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            ClearErrorResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            ClearErrorResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_addr_enabled(&self, param_bus_id: i32, param_addr: i32, param_num: i32, context: &C) -> Box<Future<Item=GetAddrEnabledResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/addr/{num}/enabled",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrEnabledResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrEnabledResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrEnabledResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_addr_info(&self, param_bus_id: i32, param_addr: i32, param_num: i32, context: &C) -> Box<Future<Item=GetAddrInfoResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/addr/{num}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::AddrInfo>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrInfoResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrInfoResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrInfoResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_addr_value(&self, param_bus_id: i32, param_addr: i32, param_num: i32, context: &C) -> Box<Future<Item=GetAddrValueResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/addr/{num}/addr",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrValueResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrValueResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetAddrValueResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_api(&self, context: &C) -> Box<Future<Item=GetApiResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/api",
            self.base_path
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 Ok(body.to_string())

                                             ))
                        .map(move |body|
                            GetApiResponse::OK(models::Yaml::from(body).to_string())
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                404 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 Ok(body.to_string())

                                             ))
                        .map(move |body|
                            GetApiResponse::FileNotFound(models::Yaml::from(body).to_string())
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_config(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetConfigResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/config",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::Config>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetConfigResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetConfigResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetConfigResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_current(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetCurrentResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/current",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetCurrentResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetCurrentResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetCurrentResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_error(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetErrorResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/error",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetErrorResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetErrorResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetErrorResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_errors(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetErrorsResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/errors",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<Vec<models::AddrInfo>>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetErrorsResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetErrorsResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetErrorsResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_freq(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetFreqResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/freq",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetFreqResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetFreqResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetFreqResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_group(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetGroupResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/group",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::Group>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetGroupResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetGroupResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetGroupResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_led_current(&self, param_bus_id: i32, param_addr: i32, param_led: i32, context: &C) -> Box<Future<Item=GetLedCurrentResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/current",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedCurrentResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedCurrentResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedCurrentResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_led_error(&self, param_bus_id: i32, param_addr: i32, param_led: i32, context: &C) -> Box<Future<Item=GetLedErrorResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/error",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::LedError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedErrorResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedErrorResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedErrorResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_led_info(&self, param_bus_id: i32, param_addr: i32, param_led: i32, context: &C) -> Box<Future<Item=GetLedInfoResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::LedInfo>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedInfoResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedInfoResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedInfoResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_led_info_all(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetLedInfoAllResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<Vec<models::LedInfo>>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedInfoAllResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedInfoAllResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedInfoAllResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_led_pwm(&self, param_bus_id: i32, param_addr: i32, param_led: i32, context: &C) -> Box<Future<Item=GetLedPwmResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/pwm",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedPwmResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedPwmResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedPwmResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_led_state(&self, param_bus_id: i32, param_addr: i32, param_led: i32, context: &C) -> Box<Future<Item=GetLedStateResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/state",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::LedState>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedStateResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedStateResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetLedStateResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_offset(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetOffsetResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/offset",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOffsetResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOffsetResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOffsetResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_output_change(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetOutputChangeResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/outputChange",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OutputChange>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOutputChangeResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOutputChangeResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOutputChangeResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_over_temp(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetOverTempResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/overTemp",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOverTempResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOverTempResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetOverTempResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_pwm(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetPwmResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/pwm",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<i32>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetPwmResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetPwmResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetPwmResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn get_sleep(&self, param_bus_id: i32, param_addr: i32, context: &C) -> Box<Future<Item=GetSleepResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/sleep",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Get, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<bool>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetSleepResponse::OK(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetSleepResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            GetSleepResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn reset(&self, param_bus_id: i32, context: &C) -> Box<Future<Item=ResetResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/reset",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            ResetResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            ResetResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            ResetResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_addr_enabled(&self, param_bus_id: i32, param_addr: i32, param_num: i32, param_enabled: bool, context: &C) -> Box<Future<Item=SetAddrEnabledResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/addr/{num}/enabled/{enabled}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET), enabled=utf8_percent_encode(&param_enabled.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetAddrEnabledResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetAddrEnabledResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetAddrEnabledResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_addr_value(&self, param_bus_id: i32, param_addr: i32, param_num: i32, param_addr_val: i32, context: &C) -> Box<Future<Item=SetAddrValueResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/addr/{num}/addr/{addrVal}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), num=utf8_percent_encode(&param_num.to_string(), ID_ENCODE_SET), addrVal=utf8_percent_encode(&param_addr_val.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetAddrValueResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetAddrValueResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetAddrValueResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_config(&self, param_bus_id: i32, param_addr: i32, param_config: models::Config, context: &C) -> Box<Future<Item=SetConfigResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/config",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);


        let body = serde_json::to_string(&param_config).expect("impossible to fail to serialize");


        request.set_body(body.into_bytes());


        request.headers_mut().set(ContentType(mimetypes::requests::SET_CONFIG.clone()));
        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetConfigResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetConfigResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetConfigResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_current(&self, param_bus_id: i32, param_addr: i32, param_current: i32, context: &C) -> Box<Future<Item=SetCurrentResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/current/{current}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), current=utf8_percent_encode(&param_current.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetCurrentResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetCurrentResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetCurrentResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_freq(&self, param_bus_id: i32, param_addr: i32, param_freq: i32, context: &C) -> Box<Future<Item=SetFreqResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/freq/{freq}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), freq=utf8_percent_encode(&param_freq.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetFreqResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetFreqResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetFreqResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_group(&self, param_bus_id: i32, param_addr: i32, param_group: models::Group, context: &C) -> Box<Future<Item=SetGroupResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/group/{group}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), group=utf8_percent_encode(&param_group.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetGroupResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetGroupResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetGroupResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_led_current(&self, param_bus_id: i32, param_addr: i32, param_led: i32, param_current: i32, context: &C) -> Box<Future<Item=SetLedCurrentResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/current/{current}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET), current=utf8_percent_encode(&param_current.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetLedCurrentResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedCurrentResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedCurrentResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_led_error(&self, param_bus_id: i32, param_addr: i32, param_led: i32, param_error: models::LedError, context: &C) -> Box<Future<Item=SetLedErrorResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/error/{error}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET), error=utf8_percent_encode(&param_error.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetLedErrorResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedErrorResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedErrorResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_led_info(&self, param_bus_id: i32, param_addr: i32, param_led: i32, param_led_info: models::LedInfo, context: &C) -> Box<Future<Item=SetLedInfoResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);


        let body = serde_json::to_string(&param_led_info).expect("impossible to fail to serialize");


        request.set_body(body.into_bytes());


        request.headers_mut().set(ContentType(mimetypes::requests::SET_LED_INFO.clone()));
        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetLedInfoResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedInfoResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedInfoResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_led_info_all(&self, param_bus_id: i32, param_addr: i32, param_led_info: &Vec<models::LedInfo>, context: &C) -> Box<Future<Item=SetLedInfoAllResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);


        let body = serde_json::to_string(&param_led_info).expect("impossible to fail to serialize");


        request.set_body(body.into_bytes());


        request.headers_mut().set(ContentType(mimetypes::requests::SET_LED_INFO_ALL.clone()));
        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetLedInfoAllResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedInfoAllResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedInfoAllResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_led_pwm(&self, param_bus_id: i32, param_addr: i32, param_led: i32, param_pwm: i32, context: &C) -> Box<Future<Item=SetLedPwmResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/pwm/{pwm}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET), pwm=utf8_percent_encode(&param_pwm.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetLedPwmResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedPwmResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedPwmResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_led_state(&self, param_bus_id: i32, param_addr: i32, param_led: i32, param_state: models::LedState, context: &C) -> Box<Future<Item=SetLedStateResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/led/{led}/state/{state}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), led=utf8_percent_encode(&param_led.to_string(), ID_ENCODE_SET), state=utf8_percent_encode(&param_state.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetLedStateResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedStateResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetLedStateResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_offset(&self, param_bus_id: i32, param_addr: i32, param_offset: i32, context: &C) -> Box<Future<Item=SetOffsetResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/offset/{offset}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), offset=utf8_percent_encode(&param_offset.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetOffsetResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetOffsetResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetOffsetResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_output_change(&self, param_bus_id: i32, param_addr: i32, param_output_change: models::OutputChange, context: &C) -> Box<Future<Item=SetOutputChangeResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/outputChange/{outputChange}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), outputChange=utf8_percent_encode(&param_output_change.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetOutputChangeResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetOutputChangeResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetOutputChangeResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_pwm(&self, param_bus_id: i32, param_addr: i32, param_pwm: i32, context: &C) -> Box<Future<Item=SetPwmResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/pwm/{pwm}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), pwm=utf8_percent_encode(&param_pwm.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetPwmResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetPwmResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetPwmResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

    fn set_sleep(&self, param_bus_id: i32, param_addr: i32, param_sleep: bool, context: &C) -> Box<Future<Item=SetSleepResponse, Error=ApiError>> {


        let uri = format!(
            "{}/pca9956b/{busId}/{addr}/sleep/{sleep}",
            self.base_path, busId=utf8_percent_encode(&param_bus_id.to_string(), ID_ENCODE_SET), addr=utf8_percent_encode(&param_addr.to_string(), ID_ENCODE_SET), sleep=utf8_percent_encode(&param_sleep.to_string(), ID_ENCODE_SET)
        );

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(futures::done(Err(ApiError(format!("Unable to build URI: {}", err))))),
        };

        let mut request = hyper::Request::new(hyper::Method::Post, uri);



        request.headers_mut().set(XSpanId((context as &Has<XSpanIdString>).get().0.clone()));


        Box::new(self.client_service.call(request)
                             .map_err(|e| ApiError(format!("No response received: {}", e)))
                             .and_then(|mut response| {
            match response.status().as_u16() {
                200 => {
                    let body = response.body();
                    Box::new(

                        future::ok(
                            SetSleepResponse::OK
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                400 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::BadRequest>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetSleepResponse::BadRequest(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                502 => {
                    let body = response.body();
                    Box::new(
                        body
                        .concat2()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                        .and_then(|body| str::from_utf8(&body)
                                             .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                                             .and_then(|body|

                                                 serde_json::from_str::<models::OpError>(body)
                                                     .map_err(|e| e.into())

                                             ))
                        .map(move |body|
                            SetSleepResponse::OperationFailed(body)
                        )
                    ) as Box<Future<Item=_, Error=_>>
                },
                code => {
                    let headers = response.headers().clone();
                    Box::new(response.body()
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
                    ) as Box<Future<Item=_, Error=_>>
                }
            }
        }))

    }

}

#[derive(Debug)]
pub enum ClientInitError {
    InvalidScheme,
    InvalidUri(hyper::error::UriError),
    MissingHost,
    SslError(openssl::error::ErrorStack)
}

impl From<hyper::error::UriError> for ClientInitError {
    fn from(err: hyper::error::UriError) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl From<openssl::error::ErrorStack> for ClientInitError {
    fn from(err: openssl::error::ErrorStack) -> ClientInitError {
        ClientInitError::SslError(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &fmt::Debug).fmt(f)
    }
}

impl error::Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}
