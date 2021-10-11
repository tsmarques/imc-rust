use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = AnnounceService {
            header: Header::new(152),

            _service: Default::default(),
            _service_type: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = AnnounceService {
            header: hdr,

            _service: Default::default(),
            _service_type: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        152
    }

    #[inline(always)]
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

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._service);
        self._service_type = bfr.get_u8();

        Ok(())
    }
}
