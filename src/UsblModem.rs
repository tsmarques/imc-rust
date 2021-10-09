use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

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

impl Message for UsblModem {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UsblModem {
            header: Header::new(901),

            _name: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UsblModem {
            header: hdr,

            _name: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        901
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        901
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z = Default::default();

        self._z_units = Default::default();
    }

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._name);

        self._lat = bfr.get_f64_le();

        self._lon = bfr.get_f64_le();

        self._z = bfr.get_f32_le();

        self._z_units = bfr.get_u8();
    }
}
