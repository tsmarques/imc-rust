use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

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

impl Message for UsblAnglesExtended {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UsblAnglesExtended {
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

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UsblAnglesExtended {
            header: hdr,

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

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        898
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        898
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._target);
        self._lbearing = bfr.get_f32_le();
        self._lelevation = bfr.get_f32_le();
        self._bearing = bfr.get_f32_le();
        self._elevation = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        self._accuracy = bfr.get_f32_le();

        Ok(())
    }
}
