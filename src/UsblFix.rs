use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// This message contains the WGS-84 position of a target computed using
/// USBL.
#[derive(Default)]
pub struct UsblFix {
    /// IMC Header
    pub header: Header,

    /// Target's IMC address.
    pub _target: u16,

    /// WGS-84 Latitude.
    pub _lat: f64,

    /// WGS-84 Longitude.
    pub _lon: f64,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Target reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,
}

impl Message for UsblFix {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = UsblFix {
            header: hdr,

            _target: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z_units: 0_u8,
            _z: Default::default(),
        };

        msg.get_header()._mgid = 892;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = UsblFix {
            header: Header::new(892),

            _target: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z_units: 0_u8,
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        892
    }

    fn id(&self) -> u16 {
        892
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z_units = Default::default();

        self._z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        23
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._target);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
