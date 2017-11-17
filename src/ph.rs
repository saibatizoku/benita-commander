//! Artifacts for pH
mod errors {
    error_chain!{}
}

use std::fmt;

pub use self::errors::*;

use benita::devices::ph::commands::Command;
use benita::devices::ph::PhSensor;
use benita::network::common::{Endpoint, SocketReply, SocketRequest};
use benita::network::ph::{PhRequester, PhResponder};
use benita::network::ph::requests::*;
use benita::utilities::{create_and_bind_responder, create_and_connect_requester};

// Define PhREQ type
requester! {
    PhREQ,
     "A request (REQ) client for pH servers.",
     PhRequester,
     PhRequester,
     [
         CalibrationClear,
         CalibrationHigh,
         CalibrationLow,
         CalibrationMid,
         CalibrationState,
         CompensationGet,
         CompensationSet,
         DeviceInformation,
         Export,
         ExportInfo,
         Import,
         Find,
         LedOff,
         LedOn,
         LedState,
         ProtocolLockDisable,
         ProtocolLockEnable,
         ProtocolLockState,
         Reading,
         Status,
         Sleep,
         Slope
             ]
}

// Creates a responder with an explicit list of commands that it handles.
responder! {
    PhREP,
     "A response (REP) server for a pH device which handles a list of commands.",
     PhSensor,
     PhResponder,
     PhResponder,
     [
         CalibrationClear,
         CalibrationHigh,
         CalibrationLow,
         CalibrationMid,
         CalibrationState,
         CompensationGet,
         CompensationSet,
         DeviceInformation,
         Export,
         ExportInfo,
         Import,
         Find,
         LedOff,
         LedOn,
         LedState,
         ProtocolLockDisable,
         ProtocolLockEnable,
         ProtocolLockState,
         Reading,
         Status,
         Sleep,
         Slope
     ]
}
