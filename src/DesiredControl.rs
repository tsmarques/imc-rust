use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Value of X is meaningful
    FL_X = 0x01,
    // Value of Y is meaningful
    FL_Y = 0x02,
    // Value of Z is meaningful
    FL_Z = 0x04,
    // Value of K is meaningful
    FL_K = 0x08,
    // Value of M is meaningful
    FL_M = 0x10,
    // Value of N is meaningful
    FL_N = 0x20,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            FL_X => 0x01,
            FL_Y => 0x02,
            FL_Z => 0x04,
            FL_K => 0x08,
            FL_M => 0x10,
            FL_N => 0x20,
        }
    }
}

/// If enabled then field M has a meaningful value.
#[derive(Default)]
pub struct DesiredControl {
    /// IMC Header
    pub header: Header,

    /// Force X along the vehicle's x axis.
    pub _x: f64,

    /// Force Y along the vehicle's y axis.
    pub _y: f64,

    /// Force Z along the vehicle's z axis.
    pub _z: f64,

    /// Torque K about the vehicle's x axis.
    pub _k: f64,

    /// Torque M about the vehicle's y axis.
    pub _m: f64,

    /// Torque N about the vehicle's z axis.
    pub _n: f64,

    /// If enabled then field N has a meaningful value.
    pub _flags: u8,
}

impl Message for DesiredControl {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DesiredControl {
            header: Header::new(407),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
            _flags: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DesiredControl {
            header: hdr,

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
            _flags: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        407
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        407
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();

        self._k = Default::default();

        self._m = Default::default();

        self._n = Default::default();

        self._flags = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        49
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
        bfr.put_f64_le(self._k);
        bfr.put_f64_le(self._m);
        bfr.put_f64_le(self._n);
        bfr.put_u8(self._flags);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._x = bfr.get_f64_le();

        self._y = bfr.get_f64_le();

        self._z = bfr.get_f64_le();

        self._k = bfr.get_f64_le();

        self._m = bfr.get_f64_le();

        self._n = bfr.get_f64_le();

        self._flags = bfr.get_u8();
    }
}
