use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// This message is sent in reply to an AcousticSystemsQuery message
/// and lists all known underwater acoustic systems (modems, narrow
/// band transponders, etc).
#[derive(Default)]
pub struct AcousticSystems {
    /// IMC Header
    pub header: Header,

    /// Comma separated list of known acoustic system names.
    pub _list: String,
}

impl Message for AcousticSystems {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = AcousticSystems {
            header: Header::new(213),

            _list: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = AcousticSystems {
            header: hdr,

            _list: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        213
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        213
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._list = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._list.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._list.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._list);
    }
}
