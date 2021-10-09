use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = ControlParcel {
            header: Header::new(412),

            _p: Default::default(),
            _i: Default::default(),
            _d: Default::default(),
            _a: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = ControlParcel {
            header: hdr,

            _p: Default::default(),
            _i: Default::default(),
            _d: Default::default(),
            _a: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        412
    }

    #[inline(always)]
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

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._p = bfr.get_f32_le();

        self._i = bfr.get_f32_le();

        self._d = bfr.get_f32_le();

        self._a = bfr.get_f32_le();
    }
}
