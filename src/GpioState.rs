use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Current state of a GPIO.
#[derive(Default)]
pub struct GpioState {
    /// IMC Header
    pub header: Header,

    /// GPIO Name.
    pub _name: String,

    /// Logical level of the GPIO.
    pub _value: u8,
}

impl Message for GpioState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = GpioState {
            header: Header::new(2000),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = GpioState {
            header: hdr,

            _name: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        2000
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2000
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);

        self._value = bfr.get_u8();

        Ok(())
    }
}
