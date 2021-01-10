use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// When the vehicle uses Long Base Line navigation, this message
/// notifies that a new range was received from one of the acoustics
/// transponders. The message fields are used to identify the range
/// value and the transponder name.
pub struct LblRange {
    /// IMC Header
    pub header: Header,

    /// Identification number of the acoustic transponder from which
    /// the range information was received.
    pub _id: u8,

    /// Distance to the acoustic transponder.
    pub _range: f32,
}

impl LblRange {
    pub fn new() -> LblRange {
        let mut msg = LblRange {
            header: Header::new(200),

            _id: Default::default(),
            _range: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LblRange {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        200
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._range = Default::default();
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

        bfr.put_u8(self._id);
        bfr.put_f32_le(self._range);

        serialize_footer(bfr);
    }
}
