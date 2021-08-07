use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

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
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = ButtonEvent {
            header: hdr,

            _button: Default::default(),
            _value: Default::default(),
        };

        msg.get_header()._mgid = 306;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = ButtonEvent {
            header: Header::new(306),

            _button: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        306
    }

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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
