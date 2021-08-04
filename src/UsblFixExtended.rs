#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// This message contains the WGS-84 position of a target computed using
/// USBL.
#[derive(Default)]
pub struct UsblFixExtended {
    /// IMC Header
    pub header: Header,

    /// Target's system name.
    pub _target: String,

    /// WGS-84 Latitude.
    pub _lat: f64,

    /// WGS-84 Longitude.
    pub _lon: f64,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Target reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Accuracy of the position fix.
    pub _accuracy: f32,
}

impl UsblFixExtended {
    pub fn new() -> UsblFixExtended {
        let mut msg = UsblFixExtended {
            header: Header::new(900),

            _target: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z_units: 0 as u8,
            _z: Default::default(),
            _accuracy: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UsblFixExtended {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        900
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z_units = Default::default();

        self._z = Default::default();

        self._accuracy = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        25
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._target.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._target.as_bytes());
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._accuracy);
    }
}
