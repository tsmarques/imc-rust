use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

use crate::packet::*;

/// message-group: Maneuver
// impl Maneuver for Teleoperation { }

/// The Teleoperation Maneuver lets the vehicle be controlled by an
/// external human operator.
/// message-group: Maneuver
#[derive(Default)]
pub struct Teleoperation {
    /// IMC Header
    pub header: Header,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Teleoperation {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Teleoperation {
            header: Header::new(452),

            _custom: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Teleoperation {
            header: hdr,

            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        452
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        452
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._custom = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._custom);
    }
}
