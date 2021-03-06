//! Artifacts for Conductivity
mod errors {
    error_chain!{}
}

pub use self::errors::*;

use benita::ezo::conductivity::device::ConductivitySensor;
use benita::ezo::conductivity::network::{ConductivityRequester, ConductivityResponder};
use benita::ezo::conductivity::network::requests::*;
use benita::ezo::devices::{I2CCommand, I2CResponse};
use benita::ezo::network::{Endpoint, SocketReply, SocketRequest};
use benita::utilities::{create_and_bind_responder, create_and_connect_requester};

// Creates a requester with an explicit list of commands that it handles.
requester! {
    ConductivityREQ,
     "A request (REQ) client for Conductivity servers which handles a list of commands.",
     ConductivityRequester,
     ConductivityRequester,
     [
         CalibrationClear,
         CalibrationDry,
         CalibrationHigh,
         CalibrationLow,
         CalibrationOnePoint,
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
         OutputDisableConductivity,
         OutputEnableConductivity,
         OutputDisableSalinity,
         OutputEnableSalinity,
         OutputDisableSpecificGravity,
         OutputEnableSpecificGravity,
         OutputDisableTds,
         OutputEnableTds,
         OutputState,
         ProbeTypeOne,
         ProbeTypePointOne,
         ProbeTypeState,
         ProbeTypeTen,
         ProtocolLockDisable,
         ProtocolLockEnable,
         ProtocolLockState,
         Reading,
         Status,
         Sleep
     ]
}

// Creates a responder with an explicit list of commands that it handles.
responder! {
    ConductivityREP,
     "A response (REP) server for a Conductivity device which handles a list of commands.",
     ConductivitySensor,
     ConductivityResponder,
     ConductivityResponder,
     [
         CalibrationClear,
         CalibrationDry,
         CalibrationHigh,
         CalibrationLow,
         CalibrationOnePoint,
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
         OutputDisableConductivity,
         OutputEnableConductivity,
         OutputDisableSalinity,
         OutputEnableSalinity,
         OutputDisableSpecificGravity,
         OutputEnableSpecificGravity,
         OutputDisableTds,
         OutputEnableTds,
         OutputState,
         ProbeTypeOne,
         ProbeTypePointOne,
         ProbeTypeState,
         ProbeTypeTen,
         ProtocolLockDisable,
         ProtocolLockEnable,
         ProtocolLockState,
         Reading,
         Status,
         Sleep
     ]
}
