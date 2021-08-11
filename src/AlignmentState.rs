use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Not Aligned
    AS_NOT_ALIGNED = 0,
    // Aligned
    AS_ALIGNED = 1,
    // Not Supported
    AS_NOT_SUPPORTED = 2,
    // Aligning
    AS_ALIGNING = 3,
    // Wrong Medium
    AS_WRONG_MEDIUM = 4,
    // Coarse Alignment
    AS_COARSE_ALIGNMENT = 5,
    // Fine Alignment
    AS_FINE_ALIGNMENT = 6,
    // System Ready
    AS_SYSTEM_READY = 7,
}

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            AS_NOT_ALIGNED => 0,
            AS_ALIGNED => 1,
            AS_NOT_SUPPORTED => 2,
            AS_ALIGNING => 3,
            AS_WRONG_MEDIUM => 4,
            AS_COARSE_ALIGNMENT => 5,
            AS_FINE_ALIGNMENT => 6,
            AS_SYSTEM_READY => 7,
        }
    }
}

/// This message notifies the vehicle is ready for dead-reckoning missions.
#[derive(Default)]
pub struct AlignmentState {
    /// IMC Header
    pub header: Header,

    /// Alignment State.
    pub _state: u8,
}

impl Message for AlignmentState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = AlignmentState {
            header: Header::new(361),

            _state: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = AlignmentState {
            header: hdr,

            _state: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        361
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        361
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._state = bfr.get_u8();
    }
}
