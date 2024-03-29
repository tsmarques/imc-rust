use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

/// Report of spatial orientation according to SNAME's notation
/// (1950).
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

impl EulerAngles {
    pub fn new() -> EulerAngles {
        let mut msg = EulerAngles {
            header: Header::new(254),

            _time: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _psi_magnetic: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EulerAngles {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        254
    }

    fn clear(&mut self) {
        self.header.clear();

        self._time = Default::default();

        self._phi = Default::default();

        self._theta = Default::default();

        self._psi = Default::default();

        self._psi_magnetic = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        40
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._phi);
        bfr.put_f64_le(self._theta);
        bfr.put_f64_le(self._psi);
        bfr.put_f64_le(self._psi_magnetic);

        serialize_footer(bfr);
    }
}
