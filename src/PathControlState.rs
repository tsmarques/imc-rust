use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Near Endpoint
    FL_NEAR = 0x01,
    // Loitering
    FL_LOITERING = 0x02,
    // No Altitude/Depth control
    FL_NO_Z = 0x04,
    // 3D Tracking
    FL_3DTRACK = 0x08,
    // Counter-Clockwise loiter
    FL_CCLOCKW = 0x10,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            FL_NEAR => 0x01,
            FL_LOITERING => 0x02,
            FL_NO_Z => 0x04,
            FL_3DTRACK => 0x08,
            FL_CCLOCKW => 0x10,
        }
    }
}

/// Indicates that loitering, if active, is being done
/// counter-clockwise. Otherwise, clockwise loitering should be
/// assumed.
#[derive(Default)]
pub struct PathControlState {
    /// IMC Header
    pub header: Header,

    /// Unsigned integer reference of the desired path message to which this
    /// PathControlState message refers to.
    /// Path reference should only be set by a maneuver, not by path controllers.
    pub _path_ref: u32,

    /// WGS-84 latitude of start point.
    pub _start_lat: f64,

    /// WGS-84 longitude of start point.
    pub _start_lon: f64,

    /// Altitude or depth of start point. This parameter will be
    /// ignored if the 'NO_Z' flag is set, or if the 'START' flag is
    /// not set.
    pub _start_z: f32,

    /// Units of the start point's z reference.
    pub _start_z_units: u8,

    /// WGS-84 latitude of end point.
    pub _end_lat: f64,

    /// WGS-84 longitude of end point.
    pub _end_lon: f64,

    /// Depth or altitude for the end point. This parameter should be
    /// ignored if the 'NO_Z' flag is set.
    pub _end_z: f32,

    /// Units of the end point's z reference.
    pub _end_z_units: u8,

    /// Radius for loitering at end point.
    /// Will be 0 if no loitering is active.
    pub _lradius: f32,

    /// 3D-tracking is active.
    pub _flags: u8,

    /// Along-Track position value.
    pub _x: f32,

    /// Cross-Track position value.
    pub _y: f32,

    /// Vertical-Track position value.
    pub _z: f32,

    /// Along-Track velocity value.
    pub _vx: f32,

    /// Cross-Track velocity value.
    pub _vy: f32,

    /// Vertical-Track velocity value.
    pub _vz: f32,

    /// Course error value.
    pub _course_error: f32,

    /// Estimated time to reach target waypoint. The value will be
    /// 65535 if the time is unknown or undefined, and 0 when
    /// loitering.
    pub _eta: u16,
}

impl Message for PathControlState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PathControlState {
            header: Header::new(410),

            _path_ref: Default::default(),
            _start_lat: Default::default(),
            _start_lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0_u8,
            _end_lat: Default::default(),
            _end_lon: Default::default(),
            _end_z: Default::default(),
            _end_z_units: 0_u8,
            _lradius: Default::default(),
            _flags: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _vx: Default::default(),
            _vy: Default::default(),
            _vz: Default::default(),
            _course_error: Default::default(),
            _eta: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PathControlState {
            header: hdr,

            _path_ref: Default::default(),
            _start_lat: Default::default(),
            _start_lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0_u8,
            _end_lat: Default::default(),
            _end_lon: Default::default(),
            _end_z: Default::default(),
            _end_z_units: 0_u8,
            _lradius: Default::default(),
            _flags: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _vx: Default::default(),
            _vy: Default::default(),
            _vz: Default::default(),
            _course_error: Default::default(),
            _eta: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        410
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        410
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._path_ref = Default::default();

        self._start_lat = Default::default();

        self._start_lon = Default::default();

        self._start_z = Default::default();

        self._start_z_units = Default::default();

        self._end_lat = Default::default();

        self._end_lon = Default::default();

        self._end_z = Default::default();

        self._end_z_units = Default::default();

        self._lradius = Default::default();

        self._flags = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();

        self._vx = Default::default();

        self._vy = Default::default();

        self._vz = Default::default();

        self._course_error = Default::default();

        self._eta = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        81
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._path_ref);
        bfr.put_f64_le(self._start_lat);
        bfr.put_f64_le(self._start_lon);
        bfr.put_f32_le(self._start_z);
        bfr.put_u8(self._start_z_units);
        bfr.put_f64_le(self._end_lat);
        bfr.put_f64_le(self._end_lon);
        bfr.put_f32_le(self._end_z);
        bfr.put_u8(self._end_z_units);
        bfr.put_f32_le(self._lradius);
        bfr.put_u8(self._flags);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._vx);
        bfr.put_f32_le(self._vy);
        bfr.put_f32_le(self._vz);
        bfr.put_f32_le(self._course_error);
        bfr.put_u16_le(self._eta);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._path_ref = bfr.get_u32_le();

        self._start_lat = bfr.get_f64_le();

        self._start_lon = bfr.get_f64_le();

        self._start_z = bfr.get_f32_le();

        self._start_z_units = bfr.get_u8();

        self._end_lat = bfr.get_f64_le();

        self._end_lon = bfr.get_f64_le();

        self._end_z = bfr.get_f32_le();

        self._end_z_units = bfr.get_u8();

        self._lradius = bfr.get_f32_le();

        self._flags = bfr.get_u8();

        self._x = bfr.get_f32_le();

        self._y = bfr.get_f32_le();

        self._z = bfr.get_f32_le();

        self._vx = bfr.get_f32_le();

        self._vy = bfr.get_f32_le();

        self._vz = bfr.get_f32_le();

        self._course_error = bfr.get_f32_le();

        self._eta = bfr.get_u16_le();
    }
}
