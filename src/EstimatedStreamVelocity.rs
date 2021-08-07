use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// The estimated stream velocity, typically for water or air
/// streams.
#[derive(Default)]
pub struct EstimatedStreamVelocity {
    /// IMC Header
    pub header: Header,

    /// X component (North).
    pub _x: f64,

    /// Y component (East).
    pub _y: f64,

    /// Z component (Down).
    pub _z: f64,
}

impl Message for EstimatedStreamVelocity {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = EstimatedStreamVelocity {
            header: hdr,

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.get_header()._mgid = 351;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = EstimatedStreamVelocity {
            header: Header::new(351),

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
        351
    }

    fn id(&self) -> u16 {
        351
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
        24
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
