use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Maneuver in progress
    MCS_EXECUTING = 0,
    // Maneuver completed
    MCS_DONE = 1,
    // Maneuver error
    MCS_ERROR = 2,
    // Maneuver stopped
    MCS_STOPPED = 3,
}

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            MCS_EXECUTING => 0,
            MCS_DONE => 1,
            MCS_ERROR => 2,
            MCS_STOPPED => 3,
        }
    }
}

/// Maneuver error.
#[derive(Default)]
pub struct ManeuverControlState {
    /// IMC Header
    pub header: Header,

    /// Maneuver stopped.
    pub _state: u8,

    /// Estimated time to completion of the maneuver, when executing.
    /// The value will be 65535 if the time is unknown or undefined.
    pub _eta: u16,

    /// Complementary information, e.g., regarding errors.
    pub _info: String,
}

impl Message for ManeuverControlState {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = ManeuverControlState {
            header: hdr,

            _state: Default::default(),
            _eta: Default::default(),
            _info: Default::default(),
        };

        msg.get_header()._mgid = 470;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = ManeuverControlState {
            header: Header::new(470),

            _state: Default::default(),
            _eta: Default::default(),
            _info: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        470
    }

    fn id(&self) -> u16 {
        470
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();

        self._eta = Default::default();

        self._info = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        3
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        bfr.put_u16_le(self._eta);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
