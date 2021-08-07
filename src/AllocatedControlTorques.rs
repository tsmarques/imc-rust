use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

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
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = AllocatedControlTorques {
            header: hdr,

            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
        };

        msg.get_header()._mgid = 411;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = AllocatedControlTorques {
            header: Header::new(411),

            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        411
    }

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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
