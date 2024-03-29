use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for Goto {}

/// A &quot;Goto&quot; is a maneuver specifying a movement of the vehicle to a
/// target waypoint. The waypoint is described by the WGS-84
/// waypoint coordinate and target Z reference.
///  
/// Mandatory parameters defined for a &quot;Goto&quot; are
/// timeout, speed and speed units.
///  
/// Optional parameters may be defined for the target Euler
/// Angles (roll, pitch and yaw) though these parameters may
/// not be considered by all maneuver controllers in charge
/// of the execution of this type of maneuver.
/// message-group: Maneuver
pub struct Goto {
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

    /// The phi Euler angle in which the vehicle should set its
    /// attitude. Use '-1' for this field to be
    /// unconsidered. Otherwise the value spans from 0 to 2 Pi.
    pub _roll: f64,

    /// The theta Euler angle in which the vehicle should set its
    /// attitude. Use '-1' for this field to be
    /// disconcerted. Otherwise the value spans from 0 to 2 Pi.
    pub _pitch: f64,

    /// The psi Euler angle in which the vehicle should set its
    /// attitude. Use '-1' for this field to be considered. Otherwise
    /// the value spans from 0 to 2 Pi.
    pub _yaw: f64,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Goto {
    pub fn new() -> Goto {
        let mut msg = Goto {
            header: Header::new(450),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0 as u8,
            _speed: Default::default(),
            _speed_units: 0 as u8,
            _roll: Default::default(),
            _pitch: Default::default(),
            _yaw: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Goto {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        450
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

        self._roll = Default::default();

        self._pitch = Default::default();

        self._yaw = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        52
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
        bfr.put_f64_le(self._roll);
        bfr.put_f64_le(self._pitch);
        bfr.put_f64_le(self._yaw);
        serialize_bytes!(bfr, self._custom.as_bytes());

        serialize_footer(bfr);
    }
}
