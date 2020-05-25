# pca9956b-api

pca9956b-api is an HTTP RESTful API designed to control a PCA9956B IC bus.  This repo includes:
- An [API specification](https://github.com/packom/pca9956b-api/blob/master/api/openapi.yaml) in [OpenAPI format](https://github.com/OAI/OpenAPI-Specification/).
- Skeleton client and server implementations in [Rust](https://www.rust-lang.org/).

A fully-featured server implementation for Linux, in Rust, can be found at https://github.com/packom/pca9956b.

The text below was automatically generated by the openapi-generator.

# Rust API for openapi_client

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

## Overview

This client/server was generated by the [openapi-generator]
(https://openapi-generator.tech) project.  By using the
[OpenAPI-Spec](https://github.com/OAI/OpenAPI-Specification) from a remote
server, you can easily generate a server stub.

To see how to make this your own, look here:

[README]((https://openapi-generator.tech))

- API version: 0.1.1
- Build date: 2020-05-25T07:48:55.283126Z[Etc/UTC]



This autogenerated project defines an API crate `openapi_client` which contains:
* An `Api` trait defining the API in Rust.
* Data types representing the underlying data model.
* A `Client` type which implements `Api` and issues HTTP requests for each operation.
* A router which accepts HTTP requests and invokes the appropriate `Api` method for each operation.

It also contains an example server and client which make use of `openapi_client`:

* The example server starts up a web server using the `openapi_client`
    router, and supplies a trivial implementation of `Api` which returns failure
    for every operation.
* The example client provides a CLI which lets you invoke
    any single operation on the `openapi_client` client by passing appropriate
    arguments on the command line.

You can use the example server and client as a basis for your own code.
See below for [more detail on implementing a server](#writing-a-server).

## Examples

Run examples with:

```
cargo run --example <example-name>
```

To pass in arguments to the examples, put them after `--`, for example:

```
cargo run --example client -- --help
```

### Running the example server
To run the server, follow these simple steps:

```
cargo run --example server
```

### Running the example client
To run a client, follow one of the following simple steps:

```
cargo run --example client ClearError
cargo run --example client GetAddrEnabled
cargo run --example client GetAddrInfo
cargo run --example client GetAddrValue
cargo run --example client GetApi
cargo run --example client GetConfig
cargo run --example client GetCurrent
cargo run --example client GetError
cargo run --example client GetErrors
cargo run --example client GetFreq
cargo run --example client GetGroup
cargo run --example client GetLedCurrent
cargo run --example client GetLedError
cargo run --example client GetLedInfo
cargo run --example client GetLedInfoAll
cargo run --example client GetLedPwm
cargo run --example client GetLedState
cargo run --example client GetOffset
cargo run --example client GetOutputChange
cargo run --example client GetOverTemp
cargo run --example client GetPwm
cargo run --example client GetSleep
cargo run --example client Reset
cargo run --example client SetAddrEnabled
cargo run --example client SetAddrValue
cargo run --example client SetCurrent
cargo run --example client SetFreq
cargo run --example client SetLedCurrent
cargo run --example client SetLedPwm
cargo run --example client SetOffset
cargo run --example client SetPwm
cargo run --example client SetSleep
```

### HTTPS
The examples can be run in HTTPS mode by passing in the flag `--https`, for example:

```
cargo run --example server -- --https
```

This will use the keys/certificates from the examples directory. Note that the
server chain is signed with `CN=localhost`.

## Using the generated library

The generated library has a few optional features that can be activated through Cargo.

* `server`
    * This defaults to enabled and creates the basic skeleton of a server implementation based on hyper
    * To create the server stack you'll need to provide an implementation of the API trait to provide the server function.
* `client`
    * This defaults to enabled and creates the basic skeleton of a client implementation based on hyper
    * The constructed client implements the API trait by making remote API call.
* `conversions`
    * This defaults to disabled and creates extra derives on models to allow "transmogrification" between objects of structurally similar types.

See https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section for how to use features in your `Cargo.toml`.

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_error**](docs/default_api.md#clear_error) | **POST** /pca9956b/{busId}/{addr}/error/clear | 
[**get_addr_enabled**](docs/default_api.md#get_addr_enabled) | **GET** /pca9956b/{busId}/{addr}/addr/{num}/enabled | 
[**get_addr_info**](docs/default_api.md#get_addr_info) | **GET** /pca9956b/{busId}/{addr}/addr/{num} | 
[**get_addr_value**](docs/default_api.md#get_addr_value) | **GET** /pca9956b/{busId}/{addr}/addr/{num}/addr | 
[**get_api**](docs/default_api.md#get_api) | **GET** /pca9956b/api | 
[**get_config**](docs/default_api.md#get_config) | **GET** /pca9956b/{busId}/{addr}/config | 
[**get_current**](docs/default_api.md#get_current) | **GET** /pca9956b/{busId}/{addr}/current | 
[**get_error**](docs/default_api.md#get_error) | **GET** /pca9956b/{busId}/{addr}/error | 
[**get_errors**](docs/default_api.md#get_errors) | **POST** /pca9956b/{busId}/{addr}/errors | 
[**get_freq**](docs/default_api.md#get_freq) | **GET** /pca9956b/{busId}/{addr}/freq | 
[**get_group**](docs/default_api.md#get_group) | **GET** /pca9956b/{busId}/{addr}/group | 
[**get_led_current**](docs/default_api.md#get_led_current) | **GET** /pca9956b/{busId}/{addr}/led/{led}/current | 
[**get_led_error**](docs/default_api.md#get_led_error) | **GET** /pca9956b/{busId}/{addr}/led/{led}/error | 
[**get_led_info**](docs/default_api.md#get_led_info) | **GET** /pca9956b/{busId}/{addr}/led/{led} | 
[**get_led_info_all**](docs/default_api.md#get_led_info_all) | **GET** /pca9956b/{busId}/{addr}/led | 
[**get_led_pwm**](docs/default_api.md#get_led_pwm) | **GET** /pca9956b/{busId}/{addr}/led/{led}/pwm | 
[**get_led_state**](docs/default_api.md#get_led_state) | **GET** /pca9956b/{busId}/{addr}/led/{led}/state | 
[**get_offset**](docs/default_api.md#get_offset) | **GET** /pca9956b/{busId}/{addr}/offset | 
[**get_output_change**](docs/default_api.md#get_output_change) | **GET** /pca9956b/{busId}/{addr}/outputChange | 
[**get_over_temp**](docs/default_api.md#get_over_temp) | **GET** /pca9956b/{busId}/{addr}/overTemp | 
[**get_pwm**](docs/default_api.md#get_pwm) | **GET** /pca9956b/{busId}/{addr}/pwm | 
[**get_sleep**](docs/default_api.md#get_sleep) | **GET** /pca9956b/{busId}/{addr}/sleep | 
[**reset**](docs/default_api.md#reset) | **POST** /pca9956b/{busId}/reset | 
[**set_addr_enabled**](docs/default_api.md#set_addr_enabled) | **POST** /pca9956b/{busId}/{addr}/addr/{num}/enabled/{enabled} | 
[**set_addr_value**](docs/default_api.md#set_addr_value) | **POST** /pca9956b/{busId}/{addr}/addr/{num}/addr/{addrVal} | 
[**set_config**](docs/default_api.md#set_config) | **POST** /pca9956b/{busId}/{addr}/config | 
[**set_current**](docs/default_api.md#set_current) | **POST** /pca9956b/{busId}/{addr}/current/{current} | 
[**set_freq**](docs/default_api.md#set_freq) | **POST** /pca9956b/{busId}/{addr}/freq/{freq} | 
[**set_group**](docs/default_api.md#set_group) | **POST** /pca9956b/{busId}/{addr}/group/{group} | 
[**set_led_current**](docs/default_api.md#set_led_current) | **POST** /pca9956b/{busId}/{addr}/led/{led}/current/{current} | 
[**set_led_error**](docs/default_api.md#set_led_error) | **POST** /pca9956b/{busId}/{addr}/led/{led}/error/{error} | 
[**set_led_info**](docs/default_api.md#set_led_info) | **POST** /pca9956b/{busId}/{addr}/led/{led} | 
[**set_led_info_all**](docs/default_api.md#set_led_info_all) | **POST** /pca9956b/{busId}/{addr}/led | 
[**set_led_pwm**](docs/default_api.md#set_led_pwm) | **POST** /pca9956b/{busId}/{addr}/led/{led}/pwm/{pwm} | 
[**set_led_state**](docs/default_api.md#set_led_state) | **POST** /pca9956b/{busId}/{addr}/led/{led}/state/{state} | 
[**set_offset**](docs/default_api.md#set_offset) | **POST** /pca9956b/{busId}/{addr}/offset/{offset} | 
[**set_output_change**](docs/default_api.md#set_output_change) | **POST** /pca9956b/{busId}/{addr}/outputChange/{outputChange} | 
[**set_pwm**](docs/default_api.md#set_pwm) | **POST** /pca9956b/{busId}/{addr}/pwm/{pwm} | 
[**set_sleep**](docs/default_api.md#set_sleep) | **POST** /pca9956b/{busId}/{addr}/sleep/{sleep} | 


## Documentation For Models

 - [Addr](docs/Addr.md)
 - [AddrEnabled](docs/AddrEnabled.md)
 - [AddrIndex](docs/AddrIndex.md)
 - [AddrInfo](docs/AddrInfo.md)
 - [ApiError](docs/ApiError.md)
 - [BadRequest](docs/BadRequest.md)
 - [BusId](docs/BusId.md)
 - [Config](docs/Config.md)
 - [Current](docs/Current.md)
 - [Error](docs/Error.md)
 - [Freq](docs/Freq.md)
 - [Group](docs/Group.md)
 - [LedError](docs/LedError.md)
 - [LedIndex](docs/LedIndex.md)
 - [LedInfo](docs/LedInfo.md)
 - [LedState](docs/LedState.md)
 - [Offset](docs/Offset.md)
 - [OpError](docs/OpError.md)
 - [OutputChange](docs/OutputChange.md)
 - [OverTemp](docs/OverTemp.md)
 - [Pwm](docs/Pwm.md)
 - [Sleep](docs/Sleep.md)
 - [Yaml](docs/Yaml.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author



