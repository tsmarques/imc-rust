#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Position and configuration of an Ultra-Short Base Line modem.
#[derive(Default)]
pub struct UsblModem {
    /// IMC Header
    pub header: Header,

    /// Name/Label of the acoustic modem.
    pub _name: String,

    /// WGS-84 Latitude coordinate.
    pub _lat: f64,

    /// WGS-84 Longitude coordinate.
    pub _lon: f64,

    /// Target reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,
}

impl UsblModem {
    pub fn new() -> UsblModem {
        let mut msg = UsblModem {
            header: Header::new(901),

            _name: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0 as u8,
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UsblModem {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        901
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z = Default::default();

        self._z_units = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        21
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
    }
}
