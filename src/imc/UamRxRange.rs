use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 817;

/// Acoustic range measurement.
pub struct UamRxRange {
    /// IMC Header
    pub header: Header,

    /// The sequence identifier of the ranging request.
    pub _seq: u16,

    /// The canonical name of the ranged system.
    pub _sys: String,

    /// The actual range. Negative values denote invalid measurements.
    pub _value: f32,
}

impl UamRxRange {
    pub fn new() -> UamRxRange {
        let mut msg = UamRxRange {
            header: Header::new(c_msg_id),

            _seq: Default::default(),
            _sys: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UamRxRange {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._sys = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u16_le(self._seq);
        serialize_bytes!(bfr, self._sys.as_bytes());
        bfr.put_f32_le(self._value);

        serialize_footer(bfr);
    }
}
