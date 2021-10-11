use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Query the brightness of an LED (Light-Emitting Diode). The
/// recipient of this message shall reply with 'LedBrightness'.
#[derive(Default)]
pub struct QueryLedBrightness {
    /// IMC Header
    pub header: Header,

    /// LED name.
    pub _name: String,
}

impl Message for QueryLedBrightness {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = QueryLedBrightness {
            header: Header::new(313),

            _name: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = QueryLedBrightness {
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
        313
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        313
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);

        Ok(())
    }
}
