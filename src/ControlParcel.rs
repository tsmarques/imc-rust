use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of PID control parcels.
#[derive(Default)]
pub struct ControlParcel {
    /// IMC Header
    pub header: Header,

    /// Proportional parcel value.
    pub _p: f32,

    /// Integral parcel value.
    pub _i: f32,

    /// Derivative parcel value.
    pub _d: f32,

    /// Anti-windup parcel value.
    pub _a: f32,
}

impl Message for ControlParcel {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = ControlParcel {
            header: hdr,

            _p: Default::default(),
            _i: Default::default(),
            _d: Default::default(),
            _a: Default::default(),
        };

        msg.get_header()._mgid = 412;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = ControlParcel {
            header: Header::new(412),

            _p: Default::default(),
            _i: Default::default(),
            _d: Default::default(),
            _a: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        412
    }

    fn id(&self) -> u16 {
        412
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._p = Default::default();

        self._i = Default::default();

        self._d = Default::default();

        self._a = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._p);
        bfr.put_f32_le(self._i);
        bfr.put_f32_le(self._d);
        bfr.put_f32_le(self._a);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
