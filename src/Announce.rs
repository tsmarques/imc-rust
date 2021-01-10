use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// A system description that is to be broadcasted to other systems.
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

impl Announce {
    pub fn new() -> Announce {
        let mut msg = Announce {
            header: Header::new(151),

            _sys_name: Default::default(),
            _sys_type: Default::default(),
            _owner: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _services: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Announce {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        151
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

    fn fixed_serialization_size(&self) -> usize {
        23
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._sys_name.len() + 2;

        dyn_size += self._services.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._sys_name.as_bytes());
        bfr.put_u8(self._sys_type);
        bfr.put_u16_le(self._owner);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        serialize_bytes!(bfr, self._services.as_bytes());

        serialize_footer(bfr);
    }
}
