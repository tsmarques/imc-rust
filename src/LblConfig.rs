use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::LblBeacon::LblBeacon;

#[allow(non_camel_case_types)]
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
#[derive(Default)]
pub struct LblConfig {
    /// IMC Header
    pub header: Header,

    /// Request the vehicle to send its current beacons configuration.
    pub _op: u8,

    /// A list of LBL beacon configuration messages.
    pub _beacons: MessageList<LblBeacon>,
}

impl LblConfig {
    pub fn new() -> LblConfig {
        let mut msg = LblConfig {
            header: Header::new(203),

            _op: Default::default(),
            _beacons: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LblConfig {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        203
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        for msg in self._beacons.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in self._beacons.iter() {
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
        bfr.put_u8(self._op);
        for msg in self._beacons.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }
}
