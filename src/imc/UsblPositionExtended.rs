use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

/// This message contains information, collected using USBL, about a
/// target's position.
pub struct UsblPositionExtended {
    /// IMC Header
    pub header: Header,

    /// Target's system name.
    pub _target: String,

    /// X coordinate of the target in the local device's reference frame.
    pub _x: f32,

    /// Y coordinate of the target in the local device's reference frame.
    pub _y: f32,

    /// Z coordinate of the target in the local device's reference frame.
    pub _z: f32,

    /// X coordinate of the target in the navigation reference frame.
    pub _n: f32,

    /// Y coordinate of the target in the navigation reference frame.
    pub _e: f32,

    /// Z coordinate of the target in the navigation reference frame.
    pub _d: f32,

    /// Rotation around the device longitudinal axis.
    pub _phi: f32,

    /// Rotation around the device lateral or transverse axis.
    pub _theta: f32,

    /// Rotation around the device vertical axis.
    pub _psi: f32,

    /// Accuracy of the position fix.
    pub _accuracy: f32,
}

impl UsblPositionExtended {
    pub fn new() -> UsblPositionExtended {
        let mut msg = UsblPositionExtended {
            header: Header::new(899),

            _target: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _n: Default::default(),
            _e: Default::default(),
            _d: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _accuracy: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UsblPositionExtended {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        899
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();

        self._n = Default::default();

        self._e = Default::default();

        self._d = Default::default();

        self._phi = Default::default();

        self._theta = Default::default();

        self._psi = Default::default();

        self._accuracy = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        40
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._target.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._target.as_bytes());
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._n);
        bfr.put_f32_le(self._e);
        bfr.put_f32_le(self._d);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._accuracy);

        serialize_footer(bfr);
    }
}
