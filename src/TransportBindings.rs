use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Message generated when tasks bind to messages.
#[derive(Default)]
pub struct TransportBindings {
    /// IMC Header
    pub header: Header,

    /// The name of the consumer (e.g. task name).
    pub _consumer: String,

    /// The id of the message to be listened to.
    pub _message_id: u16,
}

impl Message for TransportBindings {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = TransportBindings {
            header: Header::new(8),

            _consumer: Default::default(),
            _message_id: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = TransportBindings {
            header: hdr,

            _consumer: Default::default(),
            _message_id: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        8
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        8
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._consumer = Default::default();

        self._message_id = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._consumer.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._consumer.as_bytes());
        bfr.put_u16_le(self._message_id);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._consumer);

        self._message_id = bfr.get_u16_le();

        Ok(())
    }
}
