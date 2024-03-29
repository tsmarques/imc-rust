use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::MessageGroup::ControlCommand;

pub enum FlagsEnum {
    // Start Point
    FL_START = 0x01,
    // Direct
    FL_DIRECT = 0x02,
    // No Altitude/Depth control
    FL_NO_Z = 0x04,
    // 3D Tracking
    FL_3DTRACK = 0x08,
    // Counter-Clockwise loiter
    FL_CCLOCKW = 0x10,
    // Loiter from current position
    FL_LOITER_CURR = 0x20,
    // Takeoff
    FL_TAKEOFF = 0x40,
    // Land
    FL_LAND = 0x80,
}

impl FlagsEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            FL_START => 0x01,
            FL_DIRECT => 0x02,
            FL_NO_Z => 0x04,
            FL_3DTRACK => 0x08,
            FL_CCLOCKW => 0x10,
            FL_LOITER_CURR => 0x20,
            FL_TAKEOFF => 0x40,
            FL_LAND => 0x80,
        }
    }
}

/// message-group: ControlCommand
impl ControlCommand for DesiredPath {}

/// Indicates that takeoff should be done before going to the end position.
/// message-group: ControlCommand
pub struct DesiredPath {
    /// IMC Header
    pub header: Header,

    /// Unsigned integer reference for the scope of the desired path message.
    /// Path reference should only be set by a maneuver.
    /// Should be set to an always increasing reference at the time of dispatching this message.
    /// Lower level path controllers must inherit the same path reference sent by maneuver.
    pub _path_ref: u32,

    /// WGS-84 latitude of start point. This will be ignored unless
    /// the 'START' flag is set.
    pub _start_lat: f64,

    /// WGS-84 longitude of start point. This will be ignored unless
    /// the 'START' flag is set.
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

    /// Depth or altitude for the end point. This parameter will be
    /// ignored if the 'NO_Z' flag is set.
    pub _end_z: f32,

    /// Units of the end point's z reference.
    pub _end_z_units: u8,

    /// Maneuver speed reference.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Radius for loitering at end point. Specify less or equal to 0
    /// for no loitering.
    pub _lradius: f32,

    /// Indicates that the system should land at the end position.
    pub _flags: u8,
}

impl DesiredPath {
    pub fn new() -> DesiredPath {
        let mut msg = DesiredPath {
            header: Header::new(406),

            _path_ref: Default::default(),
            _start_lat: Default::default(),
            _start_lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0 as u8,
            _end_lat: Default::default(),
            _end_lon: Default::default(),
            _end_z: Default::default(),
            _end_z_units: 0 as u8,
            _speed: Default::default(),
            _speed_units: 0 as u8,
            _lradius: Default::default(),
            _flags: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredPath {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        406
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

        self._speed = Default::default();

        self._speed_units = Default::default();

        self._lradius = Default::default();

        self._flags = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        56
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u32_le(self._path_ref);
        bfr.put_f64_le(self._start_lat);
        bfr.put_f64_le(self._start_lon);
        bfr.put_f32_le(self._start_z);
        bfr.put_u8(self._start_z_units);
        bfr.put_f64_le(self._end_lat);
        bfr.put_f64_le(self._end_lon);
        bfr.put_f32_le(self._end_z);
        bfr.put_u8(self._end_z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f32_le(self._lradius);
        bfr.put_u8(self._flags);

        serialize_footer(bfr);
    }
}
