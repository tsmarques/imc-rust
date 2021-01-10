use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum FlagsEnum {
    // Promiscuous
    URF_PROMISCUOUS = 0x01,
    // Delayed
    URF_DELAYED = 0x02,
}

impl FlagsEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            URF_PROMISCUOUS => 0x01,
            URF_DELAYED => 0x02,
        }
    }
}

/// The data frame was transmitted to an acoustic modem other than
/// the one the acoustic modem driver is controlling.
pub struct UamRxFrame {
    /// IMC Header
    pub header: Header,

    /// The canonical name of the node that transmitted the data frame. If
    /// this name cannot be resolved the string 'unknown' shall be used.
    pub _sys_src: String,

    /// The canonical name of the destination node of the data frame. If
    /// this name cannot be resolved the string 'unknown' shall be used.
    pub _sys_dst: String,

    /// The data frame was transmitted using the DELAYED flag.
    pub _flags: u8,

    /// The actual received data frame.
    pub _data: Vec<u8>,
}

impl UamRxFrame {
    pub fn new() -> UamRxFrame {
        let mut msg = UamRxFrame {
            header: Header::new(815),

            _sys_src: Default::default(),
            _sys_dst: Default::default(),
            _flags: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UamRxFrame {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        815
    }

    fn clear(&mut self) {
        self.header.clear();

        self._sys_src = Default::default();

        self._sys_dst = Default::default();

        self._flags = Default::default();

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._sys_src.len() + 2;

        dyn_size += self._sys_dst.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._sys_src.as_bytes());
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._data.as_slice());

        serialize_footer(bfr);
    }
}
