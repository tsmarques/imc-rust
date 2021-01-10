use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub struct IridiumMsgTxExtended {
    /// IMC Header
    pub header: Header,

    /// The request identifier used to receive transmission updates.
    pub _req_id: u16,

    /// Time, in seconds, after which there will be no more atempts to transmit the message.
    pub _ttl: u16,

    /// Time in seconds since the Unix Epoch after which the recipient shall discard the message.
    pub _expiration: u32,

    /// The unique identifier of this message's destination (e.g. lauv-xtreme-2, manta-0).
    pub _destination: String,

    /// Message data.
    pub _data: Vec<u8>,
}

impl IridiumMsgTxExtended {
    pub fn new() -> IridiumMsgTxExtended {
        let mut msg = IridiumMsgTxExtended {
            header: Header::new(2005),

            _req_id: Default::default(),
            _ttl: Default::default(),
            _expiration: Default::default(),
            _destination: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for IridiumMsgTxExtended {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        2005
    }

    fn clear(&mut self) {
        self.header.clear();

        self._req_id = Default::default();

        self._ttl = Default::default();

        self._expiration = Default::default();

        self._destination = Default::default();

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._destination.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u16_le(self._req_id);
        bfr.put_u16_le(self._ttl);
        bfr.put_u32_le(self._expiration);
        serialize_bytes!(bfr, self._destination.as_bytes());
        serialize_bytes!(bfr, self._data.as_slice());

        serialize_footer(bfr);
    }
}
