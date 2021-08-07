use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Request the state of a given GPIO. The receiving entity shall reply
/// with a GpioState message.
#[derive(Default)]
pub struct GpioStateGet {
    /// IMC Header
    pub header: Header,

    /// GPIO Name.
    pub _name: String,
}

impl Message for GpioStateGet {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = GpioStateGet {
            header: hdr,

            _name: Default::default(),
        };

        msg.get_header()._mgid = 2001;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = GpioStateGet {
            header: Header::new(2001),

            _name: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        2001
    }

    fn id(&self) -> u16 {
        2001
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
