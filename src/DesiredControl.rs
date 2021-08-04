#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

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
    pub fn as_primitive(&self) -> u32 {
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

impl DesiredControl {
    pub fn new() -> DesiredControl {
        let mut msg = DesiredControl {
            header: Header::new(407),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
            _flags: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        407
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

    fn fixed_serialization_size(&self) -> usize {
        49
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
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
}
