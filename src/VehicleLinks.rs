#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

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

impl VehicleLinks {
    pub fn new() -> VehicleLinks {
        let mut msg = VehicleLinks {
            header: Header::new(650),

            _localname: Default::default(),
            _links: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for VehicleLinks {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        650
    }

    fn clear(&mut self) {
        self.header.clear();

        self._localname = Default::default();

        for msg in self._links.iter_mut() {
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

        dyn_size += self._localname.len() + 2;

        for msg in self._links.iter() {
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
        serialize_bytes!(bfr, self._localname.as_bytes());
        for msg in self._links.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }
}
