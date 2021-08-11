use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Water Density report.
#[derive(Default)]
pub struct WaterDensity {
    /// IMC Header
    pub header: Header,

    /// Computed Water Density.
    pub _value: f32,
}

impl Message for WaterDensity {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = WaterDensity {
            header: Header::new(268),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = WaterDensity {
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
        268
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        268
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
