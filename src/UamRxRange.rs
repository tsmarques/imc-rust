use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

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

impl Message for UamRxRange {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UamRxRange {
            header: Header::new(817),

            _seq: Default::default(),
            _sys: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UamRxRange {
            header: hdr,

            _seq: Default::default(),
            _sys: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        817
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        817
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._sys = Default::default();

        self._value = Default::default();
    }

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u16_le();

        deserialize_string!(bfr, self._sys);

        self._value = bfr.get_f32_le();

        Ok(())
    }
}
