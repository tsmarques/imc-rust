#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// This message contains information, collected using USBL, about the
/// bearing and elevation of a target.
#[derive(Default)]
pub struct UsblAnglesExtended {
    /// IMC Header
    pub header: Header,

    /// Target's system name.
    pub _target: String,

    /// Target's bearing in the local device's reference frame.
    pub _lbearing: f32,

    /// Target's elevation in the local device's reference frame.
    pub _lelevation: f32,

    /// Target's bearing in the navigation reference frame.
    pub _bearing: f32,

    /// Target's elevation in the navigation reference frame.
    pub _elevation: f32,

    /// Rotation around the device longitudinal axis.
    pub _phi: f32,

    /// Rotation around the device lateral or transverse axis.
    pub _theta: f32,

    /// Rotation around the device vertical axis.
    pub _psi: f32,

    /// Accuracy of the fix.
    pub _accuracy: f32,
}

impl UsblAnglesExtended {
    pub fn new() -> UsblAnglesExtended {
        let mut msg = UsblAnglesExtended {
            header: Header::new(898),

            _target: Default::default(),
            _lbearing: Default::default(),
            _lelevation: Default::default(),
            _bearing: Default::default(),
            _elevation: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _accuracy: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UsblAnglesExtended {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        898
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._lbearing = Default::default();

        self._lelevation = Default::default();

        self._bearing = Default::default();

        self._elevation = Default::default();

        self._phi = Default::default();

        self._theta = Default::default();

        self._psi = Default::default();

        self._accuracy = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        32
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._target.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._target.as_bytes());
        bfr.put_f32_le(self._lbearing);
        bfr.put_f32_le(self._lelevation);
        bfr.put_f32_le(self._bearing);
        bfr.put_f32_le(self._elevation);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._accuracy);
    }
}
