use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum TypeofvelocityEnum {
    // Ground velocity
    TYPE_GV = 0x01,
    // Water velocity
    TYPE_WV = 0x02,
}

impl TypeofvelocityEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            TYPE_GV => 0x01,
            TYPE_WV => 0x02,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum ReasonEnum {
    // Innovation Threshold - X
    RR_INNOV_THRESHOLD_X = 0,
    // Innovation Threshold - Y
    RR_INNOV_THRESHOLD_Y = 1,
    // Absolute Threshold - X
    RR_ABS_THRESHOLD_X = 2,
    // Absolute Threshold - Y
    RR_ABS_THRESHOLD_Y = 3,
}

impl ReasonEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            RR_INNOV_THRESHOLD_X => 0,
            RR_INNOV_THRESHOLD_Y => 1,
            RR_ABS_THRESHOLD_X => 2,
            RR_ABS_THRESHOLD_Y => 3,
        }
    }
}

/// The current DVL x-axis measurement is discarded
/// because the the absolute value is above a
/// configurable threshold.
#[derive(Default)]
pub struct DvlRejection {
    /// IMC Header
    pub header: Header,

    /// This field represents the type of the rejected velocity.
    pub _type: u8,

    /// The current DVL y-axis measurement is discarded
    /// because the the absolute value is above a
    /// configurable threshold.
    pub _reason: u8,

    /// Value of the rejection.
    /// If it is an innovation rejection the value is
    /// the absolute difference between the previous
    /// accepted DVL measurement and the current one.
    /// If it is an absolute rejection the value is
    /// the current DVL measurement.
    pub _value: f32,

    /// Timestep of the rejection.
    /// The timestep is 0 for an absolute rejection
    /// since it is an instantaneous reading. For
    /// innovation rejection it is the time difference
    /// between the previous accepted DVL measurement
    /// and the current one.
    pub _timestep: f32,
}

impl DvlRejection {
    pub fn new() -> DvlRejection {
        let mut msg = DvlRejection {
            header: Header::new(358),

            _type: Default::default(),
            _reason: Default::default(),
            _value: Default::default(),
            _timestep: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DvlRejection {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        358
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._reason = Default::default();

        self._value = Default::default();

        self._timestep = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        10
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._reason);
        bfr.put_f32_le(self._value);
        bfr.put_f32_le(self._timestep);
    }
}
