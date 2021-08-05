use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

#[allow(non_camel_case_types)]
pub enum LoiterTypeEnum {
    // Default
    LT_DEFAULT = 0,
    // Circular
    LT_CIRCULAR = 1,
    // Race track
    LT_RACETRACK = 2,
    // Figure 8
    LT_EIGHT = 3,
    // Hover
    LT_HOVER = 4,
}

impl LoiterTypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            LT_DEFAULT => 0,
            LT_CIRCULAR => 1,
            LT_RACETRACK => 2,
            LT_EIGHT => 3,
            LT_HOVER => 4,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum DirectionEnum {
    // Vehicle Dependent
    LD_VDEP = 0,
    // Clockwise
    LD_CLOCKW = 1,
    // Counter Clockwise
    LD_CCLOCKW = 2,
    // Into the wind/current
    LD_IWINDCURR = 3,
}

impl DirectionEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            LD_VDEP => 0,
            LD_CLOCKW => 1,
            LD_CCLOCKW => 2,
            LD_IWINDCURR => 3,
        }
    }
}

/// message-group: Maneuver
impl Maneuver for Loiter {}

/// The Loiter maneuver makes the vehicle circle around a specific
/// waypoint with fixed depth reference.
/// message-group: Maneuver
#[derive(Default)]
pub struct Loiter {
    /// IMC Header
    pub header: Header,

    /// The timeout indicates the time that an error should occur if
    /// exceeded.
    pub _timeout: u16,

    /// WGS-84 Latitude coordinate.
    pub _lat: f64,

    /// WGS-84 Longitude coordinate.
    pub _lon: f64,

    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,

    /// The duration of this maneuver. Use '0' for unlimited duration
    /// time.
    pub _duration: u16,

    /// Maneuver speed reference.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Loiter maneuver type.
    pub _type: u8,

    /// Radius of the maneuver.
    pub _radius: f32,

    /// Length of the maneuver.
    pub _length: f32,

    /// Bearing of the maneuver.
    pub _bearing: f64,

    /// Desired direction.
    pub _direction: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Loiter {
    pub fn new() -> Loiter {
        let mut msg = Loiter {
            header: Header::new(453),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _duration: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _type: Default::default(),
            _radius: Default::default(),
            _length: Default::default(),
            _bearing: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Loiter {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        453
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeout = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z = Default::default();

        self._z_units = Default::default();

        self._duration = Default::default();

        self._speed = Default::default();

        self._speed_units = Default::default();

        self._type = Default::default();

        self._radius = Default::default();

        self._length = Default::default();

        self._bearing = Default::default();

        self._direction = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        48
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
        bfr.put_u16_le(self._duration);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_u8(self._type);
        bfr.put_f32_le(self._radius);
        bfr.put_f32_le(self._length);
        bfr.put_f64_le(self._bearing);
        bfr.put_u8(self._direction);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }
}
