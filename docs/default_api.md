# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**clear_error**](default_api.md#clear_error) | **POST** /pca9956b/{busId}/{addr}/error/clear | 
**get_addr_enabled**](default_api.md#get_addr_enabled) | **GET** /pca9956b/{busId}/{addr}/addr/{num}/enabled | 
**get_addr_info**](default_api.md#get_addr_info) | **GET** /pca9956b/{busId}/{addr}/addr/{num} | 
**get_addr_value**](default_api.md#get_addr_value) | **GET** /pca9956b/{busId}/{addr}/addr/{num}/addr | 
**get_api**](default_api.md#get_api) | **GET** /pca9956b/api | 
**get_config**](default_api.md#get_config) | **GET** /pca9956b/{busId}/{addr}/config | 
**get_current**](default_api.md#get_current) | **GET** /pca9956b/{busId}/{addr}/current | 
**get_error**](default_api.md#get_error) | **GET** /pca9956b/{busId}/{addr}/error | 
**get_errors**](default_api.md#get_errors) | **POST** /pca9956b/{busId}/{addr}/errors | 
**get_freq**](default_api.md#get_freq) | **GET** /pca9956b/{busId}/{addr}/freq | 
**get_group**](default_api.md#get_group) | **GET** /pca9956b/{busId}/{addr}/group | 
**get_led_current**](default_api.md#get_led_current) | **GET** /pca9956b/{busId}/{addr}/led/{led}/current | 
**get_led_error**](default_api.md#get_led_error) | **GET** /pca9956b/{busId}/{addr}/led/{led}/error | 
**get_led_info**](default_api.md#get_led_info) | **GET** /pca9956b/{busId}/{addr}/led/{led} | 
**get_led_info_all**](default_api.md#get_led_info_all) | **GET** /pca9956b/{busId}/{addr}/led | 
**get_led_pwm**](default_api.md#get_led_pwm) | **GET** /pca9956b/{busId}/{addr}/led/{led}/pwm | 
**get_led_state**](default_api.md#get_led_state) | **GET** /pca9956b/{busId}/{addr}/led/{led}/state | 
**get_offset**](default_api.md#get_offset) | **GET** /pca9956b/{busId}/{addr}/offset | 
**get_output_change**](default_api.md#get_output_change) | **GET** /pca9956b/{busId}/{addr}/outputChange | 
**get_over_temp**](default_api.md#get_over_temp) | **GET** /pca9956b/{busId}/{addr}/overTemp | 
**get_pwm**](default_api.md#get_pwm) | **GET** /pca9956b/{busId}/{addr}/pwm | 
**get_sleep**](default_api.md#get_sleep) | **GET** /pca9956b/{busId}/{addr}/sleep | 
**reset**](default_api.md#reset) | **POST** /pca9956b/{busId}/reset | 
**set_addr_enabled**](default_api.md#set_addr_enabled) | **POST** /pca9956b/{busId}/{addr}/addr/{num}/enabled/{enabled} | 
**set_addr_value**](default_api.md#set_addr_value) | **POST** /pca9956b/{busId}/{addr}/addr/{num}/addr/{addrVal} | 
**set_config**](default_api.md#set_config) | **POST** /pca9956b/{busId}/{addr}/config | 
**set_current**](default_api.md#set_current) | **POST** /pca9956b/{busId}/{addr}/current/{current} | 
**set_freq**](default_api.md#set_freq) | **POST** /pca9956b/{busId}/{addr}/freq/{freq} | 
**set_group**](default_api.md#set_group) | **POST** /pca9956b/{busId}/{addr}/group/{group} | 
**set_led_current**](default_api.md#set_led_current) | **POST** /pca9956b/{busId}/{addr}/led/{led}/current/{current} | 
**set_led_error**](default_api.md#set_led_error) | **POST** /pca9956b/{busId}/{addr}/led/{led}/error/{error} | 
**set_led_info**](default_api.md#set_led_info) | **POST** /pca9956b/{busId}/{addr}/led/{led} | 
**set_led_info_all**](default_api.md#set_led_info_all) | **POST** /pca9956b/{busId}/{addr}/led | 
**set_led_pwm**](default_api.md#set_led_pwm) | **POST** /pca9956b/{busId}/{addr}/led/{led}/pwm/{pwm} | 
**set_led_state**](default_api.md#set_led_state) | **POST** /pca9956b/{busId}/{addr}/led/{led}/state/{state} | 
**set_offset**](default_api.md#set_offset) | **POST** /pca9956b/{busId}/{addr}/offset/{offset} | 
**set_output_change**](default_api.md#set_output_change) | **POST** /pca9956b/{busId}/{addr}/outputChange/{outputChange} | 
**set_pwm**](default_api.md#set_pwm) | **POST** /pca9956b/{busId}/{addr}/pwm/{pwm} | 
**set_sleep**](default_api.md#set_sleep) | **POST** /pca9956b/{busId}/{addr}/sleep/{sleep} | 


# **clear_error**
> clear_error(bus_id, addr)


Sets the clear LED error bit in the MODE1 register

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_addr_enabled**
> bool get_addr_enabled(bus_id, addr, num)


Gets whether this address is enabled

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **num** | **i32**| Address number (0=all call, 1-3=sub addrs) | 

### Return type

[**bool**](boolean.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_addr_info**
> models::AddrInfo get_addr_info(bus_id, addr, num)


Gets info about this address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **num** | **i32**| Address number (0=all call, 1-3=sub addrs) | 

### Return type

[**models::AddrInfo**](addrInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_addr_value**
> i32 get_addr_value(bus_id, addr, num)


Gets value of this address

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **num** | **i32**| Address number (0=all call, 1-3=sub addrs) | 

### Return type

[**i32**](integer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_api**
> String get_api()


Returns this API specification

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain, text/x-yaml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_config**
> models::Config get_config(bus_id, addr)


Gets global configuration on the addressed PCA9956B

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**models::Config**](config.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_current**
> i32 get_current(bus_id, addr)


Gets the current value from the IREFALL register

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**i32**](integer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_error**
> bool get_error(bus_id, addr)


Returns whether there's any LED errors

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**bool**](boolean.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_errors**
> Vec<models::AddrInfo> get_errors(bus_id, addr)


Gets error status for all LEDs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**Vec<models::AddrInfo>**](addrInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_freq**
> i32 get_freq(bus_id, addr)


Gets the frequency value from the GRPFRW register

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**i32**](integer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_group**
> models::Group get_group(bus_id, addr)


Gets the group control setting from the MODE1 register

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**models::Group**](group.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_led_current**
> i32 get_led_current(bus_id, addr, led)


Gets LED current

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 

### Return type

[**i32**](integer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_led_error**
> models::LedError get_led_error(bus_id, addr, led)


Gets LED error

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 

### Return type

[**models::LedError**](ledError.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_led_info**
> models::LedInfo get_led_info(bus_id, addr, led)


Gets all properties for 1 LED

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 

### Return type

[**models::LedInfo**](ledInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_led_info_all**
> Vec<models::LedInfo> get_led_info_all(bus_id, addr)


Gets properties for all LEDs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**Vec<models::LedInfo>**](ledInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_led_pwm**
> i32 get_led_pwm(bus_id, addr, led)


Gets LED pwm

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 

### Return type

[**i32**](integer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_led_state**
> models::LedState get_led_state(bus_id, addr, led)


Gets LED state

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 

### Return type

[**models::LedState**](ledState.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_offset**
> i32 get_offset(bus_id, addr)


Gets the offset value

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**i32**](integer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_output_change**
> models::OutputChange get_output_change(bus_id, addr)


Gets the output change mode from the MODE1 register

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**models::OutputChange**](outputChange.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_over_temp**
> bool get_over_temp(bus_id, addr)


Returns whether there's an overtemperature error

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**bool**](boolean.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_pwm**
> i32 get_pwm(bus_id, addr)


Gets the PWM value

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**i32**](integer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sleep**
> bool get_sleep(bus_id, addr)


Gets the sleep value

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 

### Return type

[**bool**](boolean.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **reset**
> reset(bus_id)


Initiated a software reset. Resets all devices that response to the PCA9956B SWRST procedure on the I2C bus. Note that the device may take up to 1ms to reset

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_addr_enabled**
> set_addr_enabled(bus_id, addr, num, enabled)


Sets whether this address is enabled

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **num** | **i32**| Address number (0=all call, 1-3=sub addrs) | 
  **enabled** | **bool**| Whether to this address should be enabled | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_addr_value**
> set_addr_value(bus_id, addr, num, addr_val)


Sets whether this address value - note that all call address can only be set when it is enabled

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **num** | **i32**| Address number (0=all call, 1-3=sub addrs) | 
  **addr_val** | **i32**| Value to set this address to | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_config**
> set_config(bus_id, addr, config)


Sets global configuration on the addressed PCA9956B

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **config** | [**Config**](Config.md)| Values to write | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_current**
> set_current(bus_id, addr, current)


Sets the current value using the IREFALL register.  When this register is written all per LED current values are set to this value

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **current** | **i32**| Current value | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_freq**
> set_freq(bus_id, addr, freq)


Sets the GRPFRQ value.  When in blink mode, affects blink period (with global pwm value affecting duty cycle), otherwise unused

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **freq** | **i32**| Frequency value | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_group**
> set_group(bus_id, addr, group)


Sets the group mode (dim or blink)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **group** | [****](.md)| Group value | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_led_current**
> set_led_current(bus_id, addr, led, current)


Sets LED current

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 
  **current** | **i32**| LED to perform operation on | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_led_error**
> set_led_error(bus_id, addr, led, error)


Sets LED error

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 
  **error** | [****](.md)| LED to perform operation on | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_led_info**
> set_led_info(bus_id, addr, led, led_info)


Sets properties for 1 LED

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 
  **led_info** | [**LedInfo**](LedInfo.md)| Values to write | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_led_info_all**
> set_led_info_all(bus_id, addr, led_info)


Sets properties for 1 or more LEDs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led_info** | [**ledInfo**](ledInfo.md)| Values to write | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_led_pwm**
> set_led_pwm(bus_id, addr, led, pwm)


Sets LED pwm

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 
  **pwm** | **i32**| LED to perform operation on | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_led_state**
> set_led_state(bus_id, addr, led, state)


Sets LED state

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **led** | **i32**| LED to perform operation on | 
  **state** | [****](.md)| LED to perform operation on | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_offset**
> set_offset(bus_id, addr, offset)


Sets the offset value.  Offset to use between offset times for consecutive LEDs.  Specified offset in clock cycles (125ns).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **offset** | **i32**| Offset value | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_output_change**
> set_output_change(bus_id, addr, output_change)


Sets the output change mode (on STOP or ACK)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **output_change** | [****](.md)| Output change value | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_pwm**
> set_pwm(bus_id, addr, pwm)


Sets the PWM value using the PWMALL register.  When in group dim mode this is overlaid over the per LED PWM value for all LEDs.  When in group blink mode this specifies the blink duty cycle (with frequency affecting period)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **pwm** | **i32**| PWM value | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_sleep**
> set_sleep(bus_id, addr, sleep)


Sets the sleep value

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **bus_id** | **i32**| I2C bus the target PCA9956B is on, as returned by /i2c/buslist | 
  **addr** | **i32**| Device address to perform operation on (may be a sub or all call address) | 
  **sleep** | **bool**| Sleep value | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

