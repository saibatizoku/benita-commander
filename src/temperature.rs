//! Artifacts for Temperature
mod errors {
    error_chain!{}
}

pub use self::errors::*;

use benita::ezo::devices::{I2CCommand, I2CResponse};
use benita::ezo::network::{Endpoint, SocketReply, SocketRequest};
use benita::ezo::temperature::device::TemperatureSensor;
use benita::ezo::temperature::network::{TemperatureRequester, TemperatureResponder};
use benita::ezo::temperature::network::requests::*;
use benita::utilities::{create_and_bind_responder, create_and_connect_requester};

// Define TemperatureREQ type
requester! {
    TemperatureREQ,
     "A request (REQ) client for pH servers.",
     TemperatureRequester,
     TemperatureRequester,
     [
         CalibrationClear,
         CalibrationState,
         CalibrationTemperature,
         DataloggerDisable,
         DataloggerInterval,
         DataloggerPeriod,
         DeviceInformation,
         Export,
         ExportInfo,
         Import,
         Find,
         LedOff,
         LedOn,
         LedState,
         MemoryClear,
         MemoryRecall,
         MemoryRecallLast,
         ProtocolLockDisable,
         ProtocolLockEnable,
         ProtocolLockState,
         Reading,
         ScaleCelsius,
         ScaleFahrenheit,
         ScaleKelvin,
         ScaleState,
         Status,
         Sleep
     ]
}

// Creates a responder with an explicit list of commands that it handles.
responder! {
    TemperatureREP,
     "A response (REP) server for a Temperature device which handles a list of commands.",
     TemperatureSensor,
     TemperatureResponder,
     TemperatureResponder,
     [
         CalibrationClear,
         CalibrationState,
         CalibrationTemperature,
         DataloggerDisable,
         DataloggerInterval,
         DataloggerPeriod,
         DeviceInformation,
         Export,
         ExportInfo,
         Import,
         Find,
         LedOff,
         LedOn,
         LedState,
         MemoryClear,
         MemoryRecall,
         MemoryRecallLast,
         ProtocolLockDisable,
         ProtocolLockEnable,
         ProtocolLockState,
         Reading,
         ScaleCelsius,
         ScaleFahrenheit,
         ScaleKelvin,
         ScaleState,
         Status,
         Sleep
     ]
}
