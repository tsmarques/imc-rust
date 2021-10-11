use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Report of spatial orientation according to SNAME's notation
/// (1950).
#[derive(Default)]
pub struct EulerAngles {
    /// IMC Header
    pub header: Header,

    /// The device time.
    pub _time: f64,

    /// Rotation around the vehicle longitudinal axis.
    pub _phi: f64,

    /// Rotation around the vehicle lateral or transverse axis.
    pub _theta: f64,

    /// Rotation around the vehicle vertical axis. A value of 0 means
    /// the vehicle is oriented towards true north. In cases where the
    /// sensor cannot measure the true heading, this field will have
    /// the same value as Yaw (Magnetic).
    pub _psi: f64,

    /// Rotation around the vehicle vertical axis. A value of 0 means
    /// the vehicle is oriented towards magnetic north. In cases where
    /// the sensor cannot measure the magnetic heading, this field
    /// will have the same value as Yaw (True).
    pub _psi_magnetic: f64,
}

impl Message for EulerAngles {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = EulerAngles {
            header: Header::new(254),

            _time: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _psi_magnetic: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = EulerAngles {
            header: hdr,

            _time: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _psi_magnetic: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        254
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        254
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._time = Default::default();

        self._phi = Default::default();

        self._theta = Default::default();

        self._psi = Default::default();

        self._psi_magnetic = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        40
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._phi);
        bfr.put_f64_le(self._theta);
        bfr.put_f64_le(self._psi);
        bfr.put_f64_le(self._psi_magnetic);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._time = bfr.get_f64_le();

        self._phi = bfr.get_f64_le();

        self._theta = bfr.get_f64_le();

        self._psi = bfr.get_f64_le();

        self._psi_magnetic = bfr.get_f64_le();

        Ok(())
    }
}
