use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for Alignment {}

/// An &quot;Alignment&quot; is a maneuver specifying a movement of the vehicle to a
/// target waypoint intended to control activation of an IMU/INS in order
/// to start aligning navigation for more precise dead reckoning operation.
///  
/// Mandatory parameters defined for a &quot;Launch&quot; are
/// timeout, speed and speed units.
/// message-group: Maneuver
pub struct Alignment {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,

    /// WGS-84 Latitude of target waypoint.
    pub _lat: f64,

    /// WGS-84 Longitude of target waypoint.
    pub _lon: f64,

    /// Maneuver speed reference.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Alignment {
    pub fn new() -> Alignment {
        let mut msg = Alignment {
            header: Header::new(495),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _speed: Default::default(),
            _speed_units: 0 as u8,
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Alignment {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        495
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeout = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._speed = Default::default();

        self._speed_units = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        23
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
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_bytes!(bfr, self._custom.as_bytes());

        serialize_footer(bfr);
    }
}
