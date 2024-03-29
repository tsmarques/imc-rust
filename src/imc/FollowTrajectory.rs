use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::TrajectoryPoint::TrajectoryPoint;

use crate::imc::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for FollowTrajectory {}

/// Maneuver constituted by a list of Trajectory Points.
/// message-group: Maneuver
pub struct FollowTrajectory {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,

    /// WGS-84 Latitude for start point.
    pub _lat: f64,

    /// WGS-84 Longitude for start point.
    pub _lon: f64,

    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Maneuver speed.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// List of trajectory points.
    pub _points: Vec<Box<TrajectoryPoint>>,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl FollowTrajectory {
    pub fn new() -> FollowTrajectory {
        let mut msg = FollowTrajectory {
            header: Header::new(463),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0 as u8,
            _speed: Default::default(),
            _speed_units: 0 as u8,
            _points: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for FollowTrajectory {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        463
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

        for msg in self._points.iter_mut() {
            msg.clear();
        }

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        28
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in &self._points {
            dyn_size += msg.dynamic_serialization_size();
        }

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
        for msg in self._points.iter() {
            msg.serialize(bfr);
        }
        serialize_bytes!(bfr, self._custom.as_bytes());

        serialize_footer(bfr);
    }
}
