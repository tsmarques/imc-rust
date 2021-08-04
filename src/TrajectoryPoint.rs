#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Waypoint coordinate of a Follow Trajectory maneuver.
#[derive(Default)]
pub struct TrajectoryPoint {
    /// IMC Header
    pub header: Header,

    /// The North offset of the North/East/Down coordinate of this
    /// point in relation to the trajectory start point.
    pub _x: f32,

    /// The East offset of the North/East/Down coordinate of this
    /// point in relation to the trajectory start point.
    pub _y: f32,

    /// The Down offset of the North/East/Down coordinate of this
    /// point in relation to the trajectory start point.
    pub _z: f32,

    /// The time offset relative to the previous trajectory point.
    pub _t: f32,
}

impl TrajectoryPoint {
    pub fn new() -> TrajectoryPoint {
        let mut msg = TrajectoryPoint {
            header: Header::new(464),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _t: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for TrajectoryPoint {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        464
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();

        self._t = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._t);
    }
}
