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
}

#[allow(non_camel_case_types)]
pub mod Flags {
    // Start from current position
    pub const FLG_CURR_POS: u32 = 0x01;
}

/// message-group: Maneuver
// impl Maneuver for Elevator { }

/// If this flag is set, lat/lon/start_z fields should be
/// ignored and current vehicle position should be considered as
/// starting point for ascent/descent.
/// message-group: Maneuver
#[derive(Default)]
pub struct Elevator {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run. If the
    /// maneuver is not completed in the amount of time specified an
    /// error will be generated.
    pub _timeout: u16,

    /// Flags of the maneuver.
    pub _flags: u8,

    /// WGS-84 Latitude.
    pub _lat: f64,

    /// WGS-84 Longitude.
    pub _lon: f64,

    /// Altitude or depth of start point. This parameter will be
    /// ignored if the 'NO_Z' flag is set, or if the 'START' flag is
    /// not set.
    pub _start_z: f32,

    /// Units of the start point's z reference.
    pub _start_z_units: u8,

    /// Depth or altitude for the end point.  This parameter will be
    /// ignored if the 'NO_Z' flag is set.
    pub _end_z: f32,

    /// Units of the end point's z reference.
    pub _end_z_units: u8,

    /// Radius for use in ascent/descent. If 0 the controller to
    /// should use a custom control strategy.
    pub _radius: f32,

    /// Maneuver speed.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Elevator {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Elevator {
            header: Header::new(462),

            _timeout: Default::default(),
            _flags: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0_u8,
            _end_z: Default::default(),
            _end_z_units: 0_u8,
            _radius: Default::default(),
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
        let msg = Elevator {
            header: hdr,

            _timeout: Default::default(),
            _flags: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0_u8,
            _end_z: Default::default(),
            _end_z_units: 0_u8,
            _radius: Default::default(),
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
        462
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        462
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeout = Default::default();
        self._flags = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._start_z = Default::default();
        self._start_z_units = Default::default();
        self._end_z = Default::default();
        self._end_z_units = Default::default();
        self._radius = Default::default();
        self._speed = Default::default();
        self._speed_units = Default::default();
        self._custom = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        38
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_u8(self._flags);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._start_z);
        bfr.put_u8(self._start_z_units);
        bfr.put_f32_le(self._end_z);
        bfr.put_u8(self._end_z_units);
        bfr.put_f32_le(self._radius);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        self._flags = bfr.get_u8();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._start_z = bfr.get_f32_le();
        self._start_z_units = bfr.get_u8();
        self._end_z = bfr.get_f32_le();
        self._end_z_units = bfr.get_u8();
        self._radius = bfr.get_f32_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        deserialize_string!(bfr, self._custom);

        Ok(())
    }
}
