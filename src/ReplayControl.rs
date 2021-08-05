use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            ROP_START => 0,
            ROP_STOP => 1,
            ROP_PAUSE => 2,
            ROP_RESUME => 3,
        }
    }
}

/// Control replay of LSF logged data.
#[derive(Default)]
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
            header: Header::new(105),

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
        105
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._file = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._file.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._file.as_bytes());
    }
}
