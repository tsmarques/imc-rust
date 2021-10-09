use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// Dissolved Oxygen measurement.
#[derive(Default)]
pub struct DissolvedOxygen {
    /// IMC Header
    pub header: Header,

    /// Dissolved Oxygen reading.
    pub _value: f32,
}

impl Message for DissolvedOxygen {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DissolvedOxygen {
            header: Header::new(295),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DissolvedOxygen {
            header: hdr,

            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        295
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        295
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_f32_le();
    }
}
