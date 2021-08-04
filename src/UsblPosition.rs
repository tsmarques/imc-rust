use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// This message contains information, collected using USBL, about a
/// target's position.
#[derive(Default)]
pub struct UsblPosition {
    /// IMC Header
    pub header: Header,

    /// Target's IMC address.
    pub _target: u16,

    /// X coordinate of the target in the local device's reference frame.
    pub _x: f32,

    /// Y coordinate of the target in the local device's reference frame.
    pub _y: f32,

    /// Z coordinate of the target in the local device's reference frame.
    pub _z: f32,
}

impl UsblPosition {
    pub fn new() -> UsblPosition {
        let mut msg = UsblPosition {
            header: Header::new(891),

            _target: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UsblPosition {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        891
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        14
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._target);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
    }
}
