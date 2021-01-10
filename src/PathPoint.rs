use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Waypoint coordinate of a Follow Path maneuver.
pub struct PathPoint {
    /// IMC Header
    pub header: Header,

    /// The North offset of the North/East/Down coordinate of this
    /// point in relation to the path start point.
    pub _x: f32,

    /// The East offset of the North/East/Down coordinate of this
    /// point in relation to the path start point.
    pub _y: f32,

    /// The Down offset of the North/East/Down coordinate of this
    /// point in relation to the path start point.
    pub _z: f32,
}

impl PathPoint {
    pub fn new() -> PathPoint {
        let mut msg = PathPoint {
            header: Header::new(458),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PathPoint {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        458
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        12
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);

        serialize_footer(bfr);
    }
}
