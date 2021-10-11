use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Start from current position
    FLG_CURR_POS = 0x01,
    // Wait at surface
    FLG_WAIT_AT_SURFACE = 0x02,
    // Station keeping
    FLG_STATION_KEEP = 0x04,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            FLG_CURR_POS => 0x01,
            FLG_WAIT_AT_SURFACE => 0x02,
            FLG_STATION_KEEP => 0x04,
        }
    }
}

/// message-group: Maneuver
// impl Maneuver for PopUp { }

/// This flag will only make sense if WAIT_AT_SURFACE is also set.
/// While holding position at surface the vehicle will assume a
/// station keeping behavior.
/// message-group: Maneuver
#[derive(Default)]
pub struct PopUp {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run. If the
    /// maneuver is not completed in the amount of time specified an
    /// error will be generated.
    pub _timeout: u16,

    /// WGS-84 Latitude.
    pub _lat: f64,

    /// WGS-84 Longitude.
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

    /// The duration of this maneuver at surface level.
    /// Only used if flag WAIT_AT_SURFACE is on.
    pub _duration: u16,

    /// Radius of the maneuver.
    /// Only used if flag STATION_KEEP is on.
    pub _radius: f32,

    /// If this flag is set, duration field is used to hold position at surface
    /// for that amount of time.
    pub _flags: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for PopUp {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PopUp {
            header: Header::new(451),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _duration: Default::default(),
            _radius: Default::default(),
            _flags: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PopUp {
            header: hdr,

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _duration: Default::default(),
            _radius: Default::default(),
            _flags: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        451
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        451
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

        self._duration = Default::default();

        self._radius = Default::default();

        self._flags = Default::default();

        self._custom = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        35
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
        bfr.put_u16_le(self._duration);
        bfr.put_f32_le(self._radius);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();

        self._lat = bfr.get_f64_le();

        self._lon = bfr.get_f64_le();

        self._z = bfr.get_f32_le();

        self._z_units = bfr.get_u8();

        self._speed = bfr.get_f32_le();

        self._speed_units = bfr.get_u8();

        self._duration = bfr.get_u16_le();

        self._radius = bfr.get_f32_le();

        self._flags = bfr.get_u8();

        deserialize_string!(bfr, self._custom);

        Ok(())
    }
}
