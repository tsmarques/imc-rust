use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// When the vehicle uses Long Base Line navigation, this message
/// notifies that a new range was received from one of the acoustics
/// transponders. The message fields are used to identify the range
/// value and the transponder name.
#[derive(Default)]
pub struct LblRange {
    /// IMC Header
    pub header: Header,

    /// Identification number of the acoustic transponder from which
    /// the range information was received.
    pub _id: u8,

    /// Distance to the acoustic transponder.
    pub _range: f32,
}

impl Message for LblRange {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = LblRange {
            header: Header::new(200),

            _id: Default::default(),
            _range: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = LblRange {
            header: hdr,

            _id: Default::default(),
            _range: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        200
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        200
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._range = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_f32_le(self._range);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._id = bfr.get_u8();

        self._range = bfr.get_f32_le();

        Ok(())
    }
}
