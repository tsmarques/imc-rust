use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Event of a specific hardware button.
#[derive(Default)]
pub struct ButtonEvent {
    /// IMC Header
    pub header: Header,

    /// Button identifier.
    pub _button: u8,

    /// Value of the button.
    pub _value: u8,
}

impl Message for ButtonEvent {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = ButtonEvent {
            header: Header::new(306),

            _button: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = ButtonEvent {
            header: hdr,

            _button: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        306
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        306
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._button = Default::default();
        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._button);
        bfr.put_u8(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._button = bfr.get_u8();
        self._value = bfr.get_u8();

        Ok(())
    }
}
