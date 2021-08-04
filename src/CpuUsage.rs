use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of software CPU usage.
#[derive(Default)]
pub struct CpuUsage {
    /// IMC Header
    pub header: Header,

    /// The CPU usage, in percentage, of the sending software.
    pub _value: u8,
}

impl CpuUsage {
    pub fn new() -> CpuUsage {
        let mut msg = CpuUsage {
            header: Header::new(7),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for CpuUsage {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        7
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._value);
    }
}
