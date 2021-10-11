use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::LblBeacon::LblBeacon;

use crate::packet::ImcError;
use crate::packet::*;

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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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

impl Message for LblConfig {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = LblConfig {
            header: Header::new(203),

            _op: Default::default(),
            _beacons: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = LblConfig {
            header: hdr,

            _op: Default::default(),
            _beacons: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        203
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        203
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._beacons = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        message_list_serialization_size!(dyn_size, self._beacons);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_message_list!(bfr, self._beacons);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();

        self._beacons = deserialize_message_list_as::<LblBeacon>(bfr)?;

        Ok(())
    }
}
