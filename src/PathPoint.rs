use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Waypoint coordinate of a Follow Path maneuver.
#[derive(Default)]
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

impl Message for PathPoint {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = PathPoint {
            header: hdr,

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.get_header()._mgid = 458;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = PathPoint {
            header: Header::new(458),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        458
    }

    fn id(&self) -> u16 {
        458
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
