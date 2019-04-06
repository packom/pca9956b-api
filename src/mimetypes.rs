/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for ClearError
    lazy_static! {
        pub static ref CLEAR_ERROR_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for ClearError
    lazy_static! {
        pub static ref CLEAR_ERROR_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrEnabled
    lazy_static! {
        pub static ref GET_ADDR_ENABLED_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrEnabled
    lazy_static! {
        pub static ref GET_ADDR_ENABLED_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrEnabled
    lazy_static! {
        pub static ref GET_ADDR_ENABLED_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrInfo
    lazy_static! {
        pub static ref GET_ADDR_INFO_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrInfo
    lazy_static! {
        pub static ref GET_ADDR_INFO_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrInfo
    lazy_static! {
        pub static ref GET_ADDR_INFO_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrValue
    lazy_static! {
        pub static ref GET_ADDR_VALUE_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrValue
    lazy_static! {
        pub static ref GET_ADDR_VALUE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetAddrValue
    lazy_static! {
        pub static ref GET_ADDR_VALUE_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetApi
    lazy_static! {
        pub static ref GET_API_OK: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetApi
    lazy_static! {
        pub static ref GET_API_FILE_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetConfig
    lazy_static! {
        pub static ref GET_CONFIG_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetConfig
    lazy_static! {
        pub static ref GET_CONFIG_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetConfig
    lazy_static! {
        pub static ref GET_CONFIG_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetCurrent
    lazy_static! {
        pub static ref GET_CURRENT_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetCurrent
    lazy_static! {
        pub static ref GET_CURRENT_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetCurrent
    lazy_static! {
        pub static ref GET_CURRENT_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetError
    lazy_static! {
        pub static ref GET_ERROR_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetError
    lazy_static! {
        pub static ref GET_ERROR_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetError
    lazy_static! {
        pub static ref GET_ERROR_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetErrors
    lazy_static! {
        pub static ref GET_ERRORS_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetErrors
    lazy_static! {
        pub static ref GET_ERRORS_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetErrors
    lazy_static! {
        pub static ref GET_ERRORS_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetFreq
    lazy_static! {
        pub static ref GET_FREQ_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetFreq
    lazy_static! {
        pub static ref GET_FREQ_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetFreq
    lazy_static! {
        pub static ref GET_FREQ_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetGroup
    lazy_static! {
        pub static ref GET_GROUP_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetGroup
    lazy_static! {
        pub static ref GET_GROUP_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetGroup
    lazy_static! {
        pub static ref GET_GROUP_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedCurrent
    lazy_static! {
        pub static ref GET_LED_CURRENT_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedCurrent
    lazy_static! {
        pub static ref GET_LED_CURRENT_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedCurrent
    lazy_static! {
        pub static ref GET_LED_CURRENT_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedError
    lazy_static! {
        pub static ref GET_LED_ERROR_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedError
    lazy_static! {
        pub static ref GET_LED_ERROR_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedError
    lazy_static! {
        pub static ref GET_LED_ERROR_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedInfo
    lazy_static! {
        pub static ref GET_LED_INFO_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedInfo
    lazy_static! {
        pub static ref GET_LED_INFO_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedInfo
    lazy_static! {
        pub static ref GET_LED_INFO_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedInfoAll
    lazy_static! {
        pub static ref GET_LED_INFO_ALL_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedInfoAll
    lazy_static! {
        pub static ref GET_LED_INFO_ALL_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedInfoAll
    lazy_static! {
        pub static ref GET_LED_INFO_ALL_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedPwm
    lazy_static! {
        pub static ref GET_LED_PWM_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedPwm
    lazy_static! {
        pub static ref GET_LED_PWM_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedPwm
    lazy_static! {
        pub static ref GET_LED_PWM_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedState
    lazy_static! {
        pub static ref GET_LED_STATE_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedState
    lazy_static! {
        pub static ref GET_LED_STATE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetLedState
    lazy_static! {
        pub static ref GET_LED_STATE_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOffset
    lazy_static! {
        pub static ref GET_OFFSET_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOffset
    lazy_static! {
        pub static ref GET_OFFSET_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOffset
    lazy_static! {
        pub static ref GET_OFFSET_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOutputChange
    lazy_static! {
        pub static ref GET_OUTPUT_CHANGE_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOutputChange
    lazy_static! {
        pub static ref GET_OUTPUT_CHANGE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOutputChange
    lazy_static! {
        pub static ref GET_OUTPUT_CHANGE_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOverTemp
    lazy_static! {
        pub static ref GET_OVER_TEMP_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOverTemp
    lazy_static! {
        pub static ref GET_OVER_TEMP_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOverTemp
    lazy_static! {
        pub static ref GET_OVER_TEMP_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetPwm
    lazy_static! {
        pub static ref GET_PWM_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetPwm
    lazy_static! {
        pub static ref GET_PWM_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetPwm
    lazy_static! {
        pub static ref GET_PWM_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetSleep
    lazy_static! {
        pub static ref GET_SLEEP_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetSleep
    lazy_static! {
        pub static ref GET_SLEEP_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetSleep
    lazy_static! {
        pub static ref GET_SLEEP_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Reset
    lazy_static! {
        pub static ref RESET_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Reset
    lazy_static! {
        pub static ref RESET_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetAddrEnabled
    lazy_static! {
        pub static ref SET_ADDR_ENABLED_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetAddrEnabled
    lazy_static! {
        pub static ref SET_ADDR_ENABLED_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetAddrValue
    lazy_static! {
        pub static ref SET_ADDR_VALUE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetAddrValue
    lazy_static! {
        pub static ref SET_ADDR_VALUE_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetConfig
    lazy_static! {
        pub static ref SET_CONFIG_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetConfig
    lazy_static! {
        pub static ref SET_CONFIG_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetCurrent
    lazy_static! {
        pub static ref SET_CURRENT_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetCurrent
    lazy_static! {
        pub static ref SET_CURRENT_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetFreq
    lazy_static! {
        pub static ref SET_FREQ_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetFreq
    lazy_static! {
        pub static ref SET_FREQ_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetGroup
    lazy_static! {
        pub static ref SET_GROUP_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetGroup
    lazy_static! {
        pub static ref SET_GROUP_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedCurrent
    lazy_static! {
        pub static ref SET_LED_CURRENT_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedCurrent
    lazy_static! {
        pub static ref SET_LED_CURRENT_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedError
    lazy_static! {
        pub static ref SET_LED_ERROR_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedError
    lazy_static! {
        pub static ref SET_LED_ERROR_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedInfo
    lazy_static! {
        pub static ref SET_LED_INFO_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedInfo
    lazy_static! {
        pub static ref SET_LED_INFO_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedInfoAll
    lazy_static! {
        pub static ref SET_LED_INFO_ALL_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedInfoAll
    lazy_static! {
        pub static ref SET_LED_INFO_ALL_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedPwm
    lazy_static! {
        pub static ref SET_LED_PWM_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedPwm
    lazy_static! {
        pub static ref SET_LED_PWM_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedState
    lazy_static! {
        pub static ref SET_LED_STATE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetLedState
    lazy_static! {
        pub static ref SET_LED_STATE_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetOffset
    lazy_static! {
        pub static ref SET_OFFSET_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetOffset
    lazy_static! {
        pub static ref SET_OFFSET_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetOutputChange
    lazy_static! {
        pub static ref SET_OUTPUT_CHANGE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetOutputChange
    lazy_static! {
        pub static ref SET_OUTPUT_CHANGE_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetPwm
    lazy_static! {
        pub static ref SET_PWM_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetPwm
    lazy_static! {
        pub static ref SET_PWM_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetSleep
    lazy_static! {
        pub static ref SET_SLEEP_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SetSleep
    lazy_static! {
        pub static ref SET_SLEEP_OPERATION_FAILED: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for SetConfig
    lazy_static! {
        pub static ref SET_CONFIG: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for SetLedInfo
    lazy_static! {
        pub static ref SET_LED_INFO: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for SetLedInfoAll
    lazy_static! {
        pub static ref SET_LED_INFO_ALL: Mime = "application/json".parse().unwrap();
    }

}
