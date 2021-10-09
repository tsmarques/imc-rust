use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::Announce::Announce;

/// This message is sent by the TREX task which gives further information to a TREX instance about connected IMC nodes
#[derive(Default)]
pub struct VehicleLinks {
    /// IMC Header
    pub header: Header,

    /// The name of the vehicle being controlled
    pub _localname: String,

    /// A list of Announce messages with last announces heard
    pub _links: MessageList<Announce>,
}

impl Message for VehicleLinks {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = VehicleLinks {
            header: Header::new(650),

            _localname: Default::default(),
            _links: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = VehicleLinks {
            header: hdr,

            _localname: Default::default(),
            _links: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        650
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        650
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._localname = Default::default();

        self._links = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._localname.len() + 2;

        message_list_serialization_size!(dyn_size, self._links);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._localname.as_bytes());
        serialize_message_list!(bfr, self._links);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._localname);

        for m in self._links.iter_mut() {
            m.deserialize_fields(bfr);
        }
    }
}
