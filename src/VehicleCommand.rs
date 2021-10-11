use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

use crate::packet::*;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            VC_REQUEST => 0,
            VC_SUCCESS => 1,
            VC_IN_PROGRESS => 2,
            VC_FAILURE => 3,
        }
    }
}

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            VC_EXEC_MANEUVER => 0,
            VC_STOP_MANEUVER => 1,
            VC_START_CALIBRATION => 2,
            VC_STOP_CALIBRATION => 3,
        }
    }
}

/// Start calibrating vehicle.
#[derive(Default)]
pub struct VehicleCommand {
    /// IMC Header
    pub header: Header,

    pub _type: u8,

    /// Request ID
    pub _request_id: u16,

    /// Stop calibrating vehicle.
    pub _command: u8,

    /// Maneuver to be executed (for 'EXEC_MANEUVER' command)
    pub _maneuver: Option<Box<dyn Message>>,

    /// Amount of time to calibrate
    pub _calib_time: u16,

    /// Complementary human-readable information for replies.
    pub _info: String,
}

impl Message for VehicleCommand {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = VehicleCommand {
            header: Header::new(501),

            _type: Default::default(),
            _request_id: Default::default(),
            _command: Default::default(),
            _maneuver: Default::default(),
            _calib_time: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = VehicleCommand {
            header: hdr,

            _type: Default::default(),
            _request_id: Default::default(),
            _command: Default::default(),
            _maneuver: Default::default(),
            _calib_time: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        501
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        501
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._request_id = Default::default();

        self._command = Default::default();

        self._maneuver = Default::default();

        self._calib_time = Default::default();

        self._info = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        inline_message_serialization_size!(dyn_size, self._maneuver);

        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u16_le(self._request_id);
        bfr.put_u8(self._command);
        serialize_inline_message!(bfr, self._maneuver);
        bfr.put_u16_le(self._calib_time);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._type = bfr.get_u8();

        self._request_id = bfr.get_u16_le();

        self._command = bfr.get_u8();

        self._maneuver = deserialize_inline(bfr).ok();

        self._calib_time = bfr.get_u16_le();

        deserialize_string!(bfr, self._info);
    }
}
