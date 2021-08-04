#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum ServiceTypeEnum {
    // External
    SRV_TYPE_EXTERNAL = 0x01,
    // Local
    SRV_TYPE_LOCAL = 0x02,
}

impl ServiceTypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            SRV_TYPE_EXTERNAL => 0x01,
            SRV_TYPE_LOCAL => 0x02,
        }
    }
}

/// Announcement about the existence of a service.
#[derive(Default)]
pub struct AnnounceService {
    /// IMC Header
    pub header: Header,

    /// Semicolon separated list of URLs (see :ref:`Announce`).
    pub _service: String,

    /// Informs about the availability of the service on internal and
    /// external networks.
    pub _service_type: u8,
}

impl AnnounceService {
    pub fn new() -> AnnounceService {
        let mut msg = AnnounceService {
            header: Header::new(152),

            _service: Default::default(),
            _service_type: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for AnnounceService {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        152
    }

    fn clear(&mut self) {
        self.header.clear();

        self._service = Default::default();

        self._service_type = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._service.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._service.as_bytes());
        bfr.put_u8(self._service_type);
    }
}
