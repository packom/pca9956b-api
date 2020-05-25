# Config

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sleep** | **bool** | whether sleep is enabled (oscillator not running when sleep is enabled, and it takes >= 500us to bring the device out of sleep) | [optional] [default to None]
**group** | [***models::Group**](group.md) |  | [optional] [default to None]
**output_change** | [***models::OutputChange**](outputChange.md) |  | [optional] [default to None]
**pwm** | **u32** | PWM value, 0-255 inclusive | [optional] [default to None]
**freq** | **u32** | frequency multipier, 0-255 inclusive | [optional] [default to None]
**offset** | **u32** | offset value, number of (125ns) clock cycles between on times of consecutive LEDs | [optional] [default to None]
**current** | **u32** | current value | [optional] [default to None]
**addr** | [**Vec<models::AddrInfo>**](addrInfo.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


