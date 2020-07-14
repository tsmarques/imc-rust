use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Received SMS data.
pub struct SmsRx {
    /// IMC Header
    pub header: Header,

    /// Number of name of the sender.
    pub _source: String,

    /// Message data.
    pub _data: Vec<u8>,
}

impl SmsRx {
    pub fn new() -> SmsRx {
        let mut msg = SmsRx {
            header: Header::new(158),

            _source: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SmsRx {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        158
    }

    fn clear(&mut self) {
        self.header.clear();

        self._source = Default::default();

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._source.as_bytes());
        serialize_bytes!(bfr, self._data.as_slice());

        serialize_footer(bfr);
    }
}
