use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum TypeEnum {
    // Input Available
    IOV_TYPE_INPUT = 1,
    // Input Error
    IOV_TYPE_INPUT_ERROR = 2,
}

impl TypeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            IOV_TYPE_INPUT => 1,
            IOV_TYPE_INPUT_ERROR => 2,
        }
    }
}

/// Notification of an I/O event.
#[derive(Default)]
pub struct IoEvent {
    /// IMC Header
    pub header: Header,

    /// Event type.
    pub _type: u8,

    /// Human-readable error message.
    pub _error: String,
}

impl Message for IoEvent {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = IoEvent {
            header: hdr,

            _type: Default::default(),
            _error: Default::default(),
        };

        msg.get_header()._mgid = 813;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = IoEvent {
            header: Header::new(813),

            _type: Default::default(),
            _error: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        813
    }

    fn id(&self) -> u16 {
        813
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._error = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
