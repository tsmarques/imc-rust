use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for StationKeeping {}

/// The Station Keeping maneuver makes the vehicle come to the surface
/// and then enter a given circular perimeter around a waypoint coordinate
/// for a certain amount of time.
/// message-group: Maneuver
pub struct StationKeeping {
    /// IMC Header
    pub header: Header,

    /// WGS-84 Latitude.
    pub _lat: f64,

    /// WGS-84 Longitude.
    pub _lon: f64,

    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Radius.
    pub _radius: f32,

    /// Duration (0 for unlimited).
    pub _duration: u16,

    /// The value of the desired speed, in the scale specified
    /// by the &quot;Speed Units&quot; field.
    pub _speed: f32,

    /// Indicates the units used for the speed value.
    pub _speed_units: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl StationKeeping {
    pub fn new() -> StationKeeping {
        let mut msg = StationKeeping {
            header: Header::new(461),

            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0 as u8,
            _radius: Default::default(),
            _duration: Default::default(),
            _speed: Default::default(),
            _speed_units: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for StationKeeping {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        461
    }

    fn clear(&mut self) {
        self.header.clear();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z = Default::default();

        self._z_units = Default::default();

        self._radius = Default::default();

        self._duration = Default::default();

        self._speed = Default::default();

        self._speed_units = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        32
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._radius);
        bfr.put_u16_le(self._duration);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_bytes!(bfr, self._custom.as_bytes());

        serialize_footer(bfr);
    }
}
