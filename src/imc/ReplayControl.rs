use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 105;

pub enum OperationEnum {
    // Start
    ROP_START = 0,
    // Stop
    ROP_STOP = 1,
    // Pause
    ROP_PAUSE = 2,
    // Resume
    ROP_RESUME = 3,
}

impl OperationEnum {
    pub fn as_u32(&self) -> u32 {
        match self {
            ROP_START => 0,
            ROP_STOP => 1,
            ROP_PAUSE => 2,
            ROP_RESUME => 3,
        }
    }
}

/// Control replay of LSF logged data.
pub struct ReplayControl {
    /// IMC Header
    pub header: Header,

    /// Operation to perform.
    pub _op: u8,

    /// Pathname of file to replay.
    pub _file: String,
}

impl ReplayControl {
    pub fn new() -> ReplayControl {
        let mut msg = ReplayControl {
            header: Header::new(c_msg_id),

            _op: Default::default(),
            _file: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ReplayControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._file = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._file.as_bytes());

        serialize_footer(bfr);
    }
}
