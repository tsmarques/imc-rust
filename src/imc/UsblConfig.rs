use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::UsblModem::UsblModem;

pub enum OperationEnum {
    // Set LBL Configuration
    OP_SET_CFG = 0,
    // Retrieve LBL Configuration
    OP_GET_CFG = 1,
    // Reply to a GET command
    OP_CUR_CFG = 2,
}

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            OP_SET_CFG => 0,
            OP_GET_CFG => 1,
            OP_CUR_CFG => 2,
        }
    }
}

/// Set the beacons configuration aboard the vehicle.
pub struct UsblConfig {
    /// IMC Header
    pub header: Header,

    /// Request the vehicle to send its current beacons configuration.
    pub _op: u8,

    /// A list of USBL modem configuration messages.
    pub _modems: Vec<Box<UsblModem>>,
}

impl UsblConfig {
    pub fn new() -> UsblConfig {
        let mut msg = UsblConfig {
            header: Header::new(902),

            _op: Default::default(),
            _modems: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UsblConfig {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        902
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        for msg in self._modems.iter_mut() {
            msg.clear();
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in &self._modems {
            dyn_size += msg.dynamic_serialization_size();
        }

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        for msg in self._modems.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
