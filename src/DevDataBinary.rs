use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Verbatim representation of device data in binary format.
#[derive(Default)]
pub struct DevDataBinary {
    /// IMC Header
    pub header: Header,

    /// Raw binary data as extracted directly from the device.
    pub _value: Vec<u8>,
}

impl Message for DevDataBinary {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = DevDataBinary {
            header: hdr,

            _value: Default::default(),
        };

        msg.get_header()._mgid = 274;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = DevDataBinary {
            header: Header::new(274),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        274
    }

    fn id(&self) -> u16 {
        274
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._value.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._value.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
