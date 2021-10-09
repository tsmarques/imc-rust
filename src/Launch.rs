use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for Launch {}

/// A &quot;Launch&quot; is a maneuver specifying a movement of the vehicle to a
/// target waypoint after being launched from a ship or pier.
/// The waypoint is described by the WGS-84 waypoint coordinate and
/// target Z reference.
///  
/// Mandatory parameters defined for a &quot;Launch&quot; are
/// timeout, speed and speed units.
/// message-group: Maneuver
#[derive(Default)]
pub struct Launch {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,

    /// WGS-84 Latitude of target waypoint.
    pub _lat: f64,

    /// WGS-84 Longitude of target waypoint.
    pub _lon: f64,

    /// Target reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Maneuver speed reference.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Launch {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Launch {
            header: Header::new(485),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _custom: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Launch {
            header: hdr,

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        485
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        485
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeout = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z = Default::default();

        self._z_units = Default::default();

        self._speed = Default::default();

        self._speed_units = Default::default();

        self._custom = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        28
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._timeout = bfr.get_u16_le();

        self._lat = bfr.get_f64_le();

        self._lon = bfr.get_f64_le();

        self._z = bfr.get_f32_le();

        self._z_units = bfr.get_u8();

        self._speed = bfr.get_f32_le();

        self._speed_units = bfr.get_u8();

        deserialize_string!(bfr, self._custom);
    }
}
