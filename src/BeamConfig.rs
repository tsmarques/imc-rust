use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Beam configuration of the device.
#[derive(Default)]
pub struct BeamConfig {
    /// IMC Header
    pub header: Header,

    /// Beam width of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_width: f32,

    /// Beam height of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_height: f32,
}

impl Message for BeamConfig {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = BeamConfig {
            header: Header::new(283),

            _beam_width: Default::default(),
            _beam_height: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = BeamConfig {
            header: hdr,

            _beam_width: Default::default(),
            _beam_height: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        283
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        283
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._beam_width = Default::default();
        self._beam_height = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._beam_width);
        bfr.put_f32_le(self._beam_height);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._beam_width = bfr.get_f32_le();
        self._beam_height = bfr.get_f32_le();

        Ok(())
    }
}
