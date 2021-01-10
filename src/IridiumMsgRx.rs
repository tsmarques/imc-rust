use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

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
            header: Header::new(170),

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
        170
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
        24
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._origin.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
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
