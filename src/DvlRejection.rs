use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

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

#[allow(non_camel_case_types)]
pub mod Typeofvelocity {
    // Ground velocity
    pub const _GV: u32 = 0x01;
    // Water velocity
    pub const _WV: u32 = 0x02;
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

impl Message for DvlRejection {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DvlRejection {
            header: Header::new(358),

            _type: Default::default(),
            _reason: Default::default(),
            _value: Default::default(),
            _timestep: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DvlRejection {
            header: hdr,

            _type: Default::default(),
            _reason: Default::default(),
            _value: Default::default(),
            _timestep: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        358
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        358
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();
        self._reason = Default::default();
        self._value = Default::default();
        self._timestep = Default::default();
    }

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._reason = bfr.get_u8();
        self._value = bfr.get_f32_le();
        self._timestep = bfr.get_f32_le();

        Ok(())
    }
}
