use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// A system description that is to be broadcasted to other systems.
#[derive(Default)]
pub struct Announce {
    /// IMC Header
    pub header: Header,

    /// System name.
    pub _sys_name: String,

    /// System type.
    pub _sys_type: u8,

    /// The owner IMC system ID.
    pub _owner: u16,

    /// WGS-84 Latitude. If lat=0 and lon=0 means location value is unknown.
    pub _lat: f64,

    /// WGS-84 Longitude. If lat=0 and lon=0 means location value is unknown.
    pub _lon: f64,

    /// Height above WGS-84 ellipsoid.
    pub _height: f32,

    /// Semicolon separated list of URLs. Examples of such URLs are:
    ///  
    /// - *imc+udp://192.168.106.34:6002/*
    /// - *dune://0.0.0.0/uid/1294925553839635/*
    /// - *http://192.168.106.34/dune/*.
    pub _services: String,
}

impl Message for Announce {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Announce {
            header: Header::new(151),

            _sys_name: Default::default(),
            _sys_type: Default::default(),
            _owner: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _services: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Announce {
            header: hdr,

            _sys_name: Default::default(),
            _sys_type: Default::default(),
            _owner: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _services: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        151
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        151
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._sys_name = Default::default();

        self._sys_type = Default::default();

        self._owner = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._height = Default::default();

        self._services = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        23
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._sys_name.len() + 2;

        dyn_size += self._services.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._sys_name.as_bytes());
        bfr.put_u8(self._sys_type);
        bfr.put_u16_le(self._owner);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        serialize_bytes!(bfr, self._services.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._sys_name);

        self._sys_type = bfr.get_u8();

        self._owner = bfr.get_u16_le();

        self._lat = bfr.get_f64_le();

        self._lon = bfr.get_f64_le();

        self._height = bfr.get_f32_le();

        deserialize_string!(bfr, self._services);

        Ok(())
    }
}
