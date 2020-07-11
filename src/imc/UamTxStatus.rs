use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 816;

pub enum ValueEnum {
    // Transmission Completed
    UTS_DONE = 0,
    // Transmission Failed
    UTS_FAILED = 1,
    // Transmission Canceled
    UTS_CANCELED = 2,
    // Modem is busy
    UTS_BUSY = 3,
    // Invalid address
    UTS_INV_ADDR = 4,
    // In Progress
    UTS_IP = 5,
    // Unsupported operation
    UTS_UNSUPPORTED = 6,
    // Invalid transmission size
    UTS_INV_SIZE = 7,
    // Not transducer
    UTS_NOT_TRANSDUCER = 8,
}

impl ValueEnum {
    pub fn as_u32(&self) -> u32 {
        match self {
            UTS_DONE => 0,
            UTS_FAILED => 1,
            UTS_CANCELED => 2,
            UTS_BUSY => 3,
            UTS_INV_ADDR => 4,
            UTS_IP => 5,
            UTS_UNSUPPORTED => 6,
            UTS_INV_SIZE => 7,
            UTS_NOT_TRANSDUCER => 8,
        }
    }
}

/// transducer not connected to the acoustic modem.
pub struct UamTxStatus {
    /// IMC Header
    pub header: Header,

    /// The sequence identifier of the frame transmission request.
    pub _seq: u16,

    /// The frame transmission request exceeds the MTU of the acoustic
    /// modem.
    pub _value: u8,

    /// Where applicable this field shall contain a human-readable message
    /// explaining the error.
    pub _error: String,
}

impl UamTxStatus {
    pub fn new() -> UamTxStatus {
        let mut msg = UamTxStatus {
            header: Header::new(c_msg_id),

            _seq: Default::default(),
            _value: Default::default(),
            _error: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UamTxStatus {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._value = Default::default();

        self._error = Default::default();
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
        bfr.put_u8(self._value);
        serialize_bytes!(bfr, self._error.as_bytes());

        serialize_footer(bfr);
    }
}
