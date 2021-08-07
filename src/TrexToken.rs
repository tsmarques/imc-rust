use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::TrexAttribute::TrexAttribute;

#[derive(Default)]
pub struct TrexToken {
    /// IMC Header
    pub header: Header,

    pub _timeline: String,

    pub _predicate: String,

    pub _attributes: MessageList<TrexAttribute>,
}

impl Message for TrexToken {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = TrexToken {
            header: hdr,

            _timeline: Default::default(),
            _predicate: Default::default(),
            _attributes: vec![],
        };

        msg.get_header()._mgid = 657;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = TrexToken {
            header: Header::new(657),

            _timeline: Default::default(),
            _predicate: Default::default(),
            _attributes: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        657
    }

    fn id(&self) -> u16 {
        657
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeline = Default::default();

        self._predicate = Default::default();

        for msg in self._attributes.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._timeline.len() + 2;

        dyn_size += self._predicate.len() + 2;

        for msg in self._attributes.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._timeline.as_bytes());
        serialize_bytes!(bfr, self._predicate.as_bytes());
        for msg in self._attributes.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
