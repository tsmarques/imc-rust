use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::MessageGroup::Maneuver;

pub enum FlagsEnum {
    // Square Curve
    FLG_SQUARE_CURVE = 0x0001,
    // First Curve Right
    FLG_CURVE_RIGHT = 0x0002,
}

impl FlagsEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            FLG_SQUARE_CURVE => 0x0001,
            FLG_CURVE_RIGHT => 0x0002,
        }
    }
}

/// message-group: Maneuver
impl Maneuver for Rows {}

/// Rows maneuver (i.e: lawn mower type maneuver)
/// message-group: Maneuver
pub struct Rows {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,

    /// WGS-84 Latitude of target waypoint.
    pub _lat: f64,

    /// WGS-84 Longitude of target waypoint.
    pub _lon: f64,

    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Maneuver speed reference.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Rows bearing angle.
    pub _bearing: f64,

    /// Rows cross angle reference.
    pub _cross_angle: f64,

    /// Width of the maneuver.
    pub _width: f32,

    /// Length of the maneuver.
    pub _length: f32,

    /// Horizontal step.
    pub _hstep: f32,

    /// Desired curve offset.
    pub _coff: u8,

    /// Alternation parameter.
    pub _alternation: u8,

    /// Maneuver flags.
    pub _flags: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Rows {
    pub fn new() -> Rows {
        let mut msg = Rows {
            header: Header::new(456),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0 as u8,
            _speed: Default::default(),
            _speed_units: 0 as u8,
            _bearing: Default::default(),
            _cross_angle: Default::default(),
            _width: Default::default(),
            _length: Default::default(),
            _hstep: 30 as f32,
            _coff: Default::default(),
            _alternation: 50 as u8,
            _flags: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Rows {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        456
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

        self._bearing = Default::default();

        self._cross_angle = Default::default();

        self._width = Default::default();

        self._length = Default::default();

        self._hstep = Default::default();

        self._coff = Default::default();

        self._alternation = Default::default();

        self._flags = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        59
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u16_le(self._timeout);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f64_le(self._bearing);
        bfr.put_f64_le(self._cross_angle);
        bfr.put_f32_le(self._width);
        bfr.put_f32_le(self._length);
        bfr.put_f32_le(self._hstep);
        bfr.put_u8(self._coff);
        bfr.put_u8(self._alternation);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._custom.as_bytes());

        serialize_footer(bfr);
    }
}
