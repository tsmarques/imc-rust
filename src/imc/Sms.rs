use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Send a SMS message.
pub struct Sms {
    /// IMC Header
    pub header: Header,

    /// Target mobile device number.
    pub _number: String,

    /// Timeout for sending message.
    pub _timeout: u16,

    /// Message contents.
    pub _contents: String,
}

impl Sms {
    pub fn new() -> Sms {
        let mut msg = Sms {
            header: Header::new(156),

            _number: Default::default(),
            _timeout: Default::default(),
            _contents: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Sms {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        156
    }

    fn clear(&mut self) {
        self.header.clear();

        self._number = Default::default();

        self._timeout = Default::default();

        self._contents = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._number.as_bytes());
        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._contents.as_bytes());

        serialize_footer(bfr);
    }
}
