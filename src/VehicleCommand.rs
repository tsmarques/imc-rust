use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

pub enum TypeEnum {
    // Request
    VC_REQUEST = 0,
    // Reply -- Success
    VC_SUCCESS = 1,
    // Reply -- In Progress
    VC_IN_PROGRESS = 2,
    // Reply -- Failure
    VC_FAILURE = 3,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            VC_REQUEST => 0,
            VC_SUCCESS => 1,
            VC_IN_PROGRESS => 2,
            VC_FAILURE => 3,
        }
    }
}

pub enum CommandEnum {
    // Execute Maneuver
    VC_EXEC_MANEUVER = 0,
    // Stop Maneuver
    VC_STOP_MANEUVER = 1,
    // Start Calibration
    VC_START_CALIBRATION = 2,
    // Stop Calibration
    VC_STOP_CALIBRATION = 3,
}

impl CommandEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            VC_EXEC_MANEUVER => 0,
            VC_STOP_MANEUVER => 1,
            VC_START_CALIBRATION => 2,
            VC_STOP_CALIBRATION => 3,
        }
    }
}

/// Start calibrating vehicle.
pub struct VehicleCommand {
    /// IMC Header
    pub header: Header,

    pub _type: u8,

    /// Request ID
    pub _request_id: u16,

    /// Stop calibrating vehicle.
    pub _command: u8,

    /// Maneuver to be executed (for 'EXEC_MANEUVER' command)
    pub _maneuver: Option<Box<dyn Maneuver>>,

    /// Amount of time to calibrate
    pub _calib_time: u16,

    /// Complementary human-readable information for replies.
    pub _info: String,
}

impl VehicleCommand {
    pub fn new() -> VehicleCommand {
        let mut msg = VehicleCommand {
            header: Header::new(501),

            _type: Default::default(),
            _request_id: Default::default(),
            _command: Default::default(),
            _maneuver: Default::default(),
            _calib_time: Default::default(),
            _info: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for VehicleCommand {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        501
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._request_id = Default::default();

        self._command = Default::default();

        match &mut self._maneuver {
            Some(field) => field.clear(),

            None => {}
        }

        self._calib_time = Default::default();

        self._info = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        match &self._maneuver {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._type);
        bfr.put_u16_le(self._request_id);
        bfr.put_u8(self._command);
        match &self._maneuver {
            Some(field) => field.serialize(bfr),

            None => {}
        };
        bfr.put_u16_le(self._calib_time);
        serialize_bytes!(bfr, self._info.as_bytes());

        serialize_footer(bfr);
    }
}
