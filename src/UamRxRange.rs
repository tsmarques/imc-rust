use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Acoustic range measurement.
#[derive(Default)]
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
            header: Header::new(817),

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
        817
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._sys = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._sys.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        serialize_bytes!(bfr, self._sys.as_bytes());
        bfr.put_f32_le(self._value);
    }
}
