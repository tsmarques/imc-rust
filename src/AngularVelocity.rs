use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Vector quantifying the direction and magnitude of the measured
/// angular velocity that a device is exposed to.
#[derive(Default)]
pub struct AngularVelocity {
    /// IMC Header
    pub header: Header,

    /// The device time.
    pub _time: f64,

    /// X component.
    pub _x: f64,

    /// Y component.
    pub _y: f64,

    /// Z component.
    pub _z: f64,
}

impl AngularVelocity {
    pub fn new() -> AngularVelocity {
        let mut msg = AngularVelocity {
            header: Header::new(256),

            _time: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for AngularVelocity {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        256
    }

    fn clear(&mut self) {
        self.header.clear();

        self._time = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        32
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
    }
}
