use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = GpioStateGet {
            header: Header::new(2001),

            _name: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = GpioStateGet {
            header: hdr,

            _name: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        2001
    }

    #[inline(always)]
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

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._name);
    }
}
