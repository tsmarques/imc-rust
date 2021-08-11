use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Off
    PCS_OFF = 0,
    // On
    PCS_ON = 1,
}

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            PCS_OFF => 0,
            PCS_ON => 1,
        }
    }
}

/// Power channel is off.
#[derive(Default)]
pub struct PowerChannelState {
    /// IMC Header
    pub header: Header,

    /// Power Channel Name.
    pub _name: String,

    /// Power channel is on.
    pub _state: u8,
}

impl Message for PowerChannelState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PowerChannelState {
            header: Header::new(311),

            _name: Default::default(),
            _state: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PowerChannelState {
            header: hdr,

            _name: Default::default(),
            _state: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        311
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        311
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._state = Default::default();
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
        bfr.put_u8(self._state);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._name);

        self._state = bfr.get_u8();
    }
}
