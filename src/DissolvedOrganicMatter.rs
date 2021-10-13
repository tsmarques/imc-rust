use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum TypeofmeasurementEnum {
    // Colored
    DT_COLORED = 0,
    // Fluorescent
    DT_FLUORESCENT = 1,
}

#[allow(non_camel_case_types)]
pub mod Typeofmeasurement {
    // Colored
    pub const DT_COLORED: u32 = 0;
    // Fluorescent
    pub const DT_FLUORESCENT: u32 = 1;
}

/// Dissolved Organic Matter measurement.
#[derive(Default)]
pub struct DissolvedOrganicMatter {
    /// IMC Header
    pub header: Header,

    /// Dissolved Organic Matter reading.
    pub _value: f32,

    /// Type of measurement.
    pub _type: u8,
}

impl Message for DissolvedOrganicMatter {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DissolvedOrganicMatter {
            header: Header::new(903),

            _value: Default::default(),
            _type: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DissolvedOrganicMatter {
            header: hdr,

            _value: Default::default(),
            _type: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        903
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        903
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
        self._type = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_u8(self._type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();
        self._type = bfr.get_u8();

        Ok(())
    }
}
