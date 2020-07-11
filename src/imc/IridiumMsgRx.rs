use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 170;

pub struct IridiumMsgRx {
    /// IMC Header
    pub header: Header,

    /// The unique identifier of this message's origin device (e.g. lauv-xtreme-2, manta-0).
    pub _origin: String,

    /// Timestamp (Epoch time).
    pub _htime: f64,

    pub _lat: f64,

    pub _lon: f64,

    /// Message data.
    pub _data: Vec<u8>,
}

impl IridiumMsgRx {
    pub fn new() -> IridiumMsgRx {
        let mut msg = IridiumMsgRx {
            header: Header::new(c_msg_id),

            _origin: Default::default(),
            _htime: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for IridiumMsgRx {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._origin = Default::default();

        self._htime = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._origin.as_bytes());
        bfr.put_f64_le(self._htime);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        serialize_bytes!(bfr, self._data.as_slice());

        serialize_footer(bfr);
    }
}
