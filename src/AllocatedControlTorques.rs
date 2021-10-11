use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Control torques allocated to the actuators.
#[derive(Default)]
pub struct AllocatedControlTorques {
    /// IMC Header
    pub header: Header,

    /// Torque K about the vehicle's x axis.
    pub _k: f64,

    /// Torque M about the vehicle's y axis.
    pub _m: f64,

    /// Torque N about the vehicle's z axis.
    pub _n: f64,
}

impl Message for AllocatedControlTorques {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = AllocatedControlTorques {
            header: Header::new(411),

            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = AllocatedControlTorques {
            header: hdr,

            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        411
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        411
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._k = Default::default();

        self._m = Default::default();

        self._n = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        24
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._k);
        bfr.put_f64_le(self._m);
        bfr.put_f64_le(self._n);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._k = bfr.get_f64_le();

        self._m = bfr.get_f64_le();

        self._n = bfr.get_f64_le();

        Ok(())
    }
}
