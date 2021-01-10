use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum TypeofmeasurementEnum {
    // Colored
    DT_COLORED = 0,
    // Fluorescent
    DT_FLUORESCENT = 1,
}

impl TypeofmeasurementEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            DT_COLORED => 0,
            DT_FLUORESCENT => 1,
        }
    }
}

/// Dissolved Organic Matter measurement.
pub struct DissolvedOrganicMatter {
    /// IMC Header
    pub header: Header,

    /// Dissolved Organic Matter reading.
    pub _value: f32,

    /// Type of measurement.
    pub _type: u8,
}

impl DissolvedOrganicMatter {
    pub fn new() -> DissolvedOrganicMatter {
        let mut msg = DissolvedOrganicMatter {
            header: Header::new(903),

            _value: Default::default(),
            _type: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DissolvedOrganicMatter {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        903
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
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f32_le(self._value);
        bfr.put_u8(self._type);

        serialize_footer(bfr);
    }
}
