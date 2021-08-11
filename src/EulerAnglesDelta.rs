use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Component of incremetal orientation vector over a period of time.
#[derive(Default)]
pub struct EulerAnglesDelta {
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

    /// Period of time of the orientation vector increments.
    pub _timestep: f32,
}

impl Message for EulerAnglesDelta {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = EulerAnglesDelta {
            header: Header::new(255),

            _time: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _timestep: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = EulerAnglesDelta {
            header: hdr,

            _time: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _timestep: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        255
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        255
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._time = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();

        self._timestep = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        36
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
        bfr.put_f32_le(self._timestep);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._time = bfr.get_f64_le();

        self._x = bfr.get_f64_le();

        self._y = bfr.get_f64_le();

        self._z = bfr.get_f64_le();

        self._timestep = bfr.get_f32_le();
    }
}
