use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

use crate::imc::Announce::Announce;

/// This message is sent by the TREX task which gives further information to a TREX instance about connected IMC nodes
pub struct VehicleLinks {
    /// IMC Header
    pub header: Header,

    /// The name of the vehicle being controlled
    pub _localname: String,

    /// A list of Announce messages with last announces heard
    pub _links: Vec<Box<Announce>>,
}

impl VehicleLinks {
    pub fn new() -> VehicleLinks {
        let mut msg = VehicleLinks {
            header: Header::new(650),

            _localname: Default::default(),
            _links: Default::default(),
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
            msg.clear();
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._localname.as_bytes());
        for msg in self._links.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
