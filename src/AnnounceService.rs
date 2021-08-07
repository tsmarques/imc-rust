use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum ServiceTypeEnum {
    // External
    SRV_TYPE_EXTERNAL = 0x01,
    // Local
    SRV_TYPE_LOCAL = 0x02,
}

impl ServiceTypeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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

impl Message for AnnounceService {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = AnnounceService {
            header: hdr,

            _service: Default::default(),
            _service_type: Default::default(),
        };

        msg.get_header()._mgid = 152;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = AnnounceService {
            header: Header::new(152),

            _service: Default::default(),
            _service_type: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        152
    }

    fn id(&self) -> u16 {
        152
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
