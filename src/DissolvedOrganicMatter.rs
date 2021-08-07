use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum TypeofmeasurementEnum {
    // Colored
    DT_COLORED = 0,
    // Fluorescent
    DT_FLUORESCENT = 1,
}

impl TypeofmeasurementEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            DT_COLORED => 0,
            DT_FLUORESCENT => 1,
        }
    }
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
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = DissolvedOrganicMatter {
            header: hdr,

            _value: Default::default(),
            _type: Default::default(),
        };

        msg.get_header()._mgid = 903;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = DissolvedOrganicMatter {
            header: Header::new(903),

            _value: Default::default(),
            _type: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        903
    }

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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
