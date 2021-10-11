use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Report of navigation uncertainty.
/// This is usually given by the output of the state
/// covariance matrix of an Extended Kalman Filter.
#[derive(Default)]
pub struct NavigationUncertainty {
    /// IMC Header
    pub header: Header,

    /// The North offset variance of the North/East/Down
    /// field with respect to LLH.
    pub _x: f32,

    /// The East offset variance of the North/East/Down
    /// field with respect to LLH.
    pub _y: f32,

    /// The Down offset variance of the North/East/Down
    /// field with respect to LLH.
    pub _z: f32,

    /// The phi Euler angle variance from the vehicle's attitude.
    pub _phi: f32,

    /// The theta Euler angle variance from the vehicle's attitude.
    pub _theta: f32,

    /// The psi Euler angle variance from the vehicle's attitude.
    pub _psi: f32,

    /// The angular velocity variance over body-fixed xx axis (roll).
    pub _p: f32,

    /// The angular velocity variance over body-fixed yy axis (pitch).
    pub _q: f32,

    /// The angular velocity variance over body-fixed zz axis (yaw).
    pub _r: f32,

    /// Body-fixed frame xx axis velocity variance component.
    pub _u: f32,

    /// Body-fixed frame yy axis velocity variance component.
    pub _v: f32,

    /// Body-fixed frame zz axis velocity variance component.
    pub _w: f32,

    /// The psi Euler angle bias variance from the vehicle's sensed attitude.
    pub _bias_psi: f32,

    /// The angular velocity over body-fixed zz axis bias variance from sensor.
    pub _bias_r: f32,
}

impl Message for NavigationUncertainty {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = NavigationUncertainty {
            header: Header::new(354),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _p: Default::default(),
            _q: Default::default(),
            _r: Default::default(),
            _u: Default::default(),
            _v: Default::default(),
            _w: Default::default(),
            _bias_psi: Default::default(),
            _bias_r: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = NavigationUncertainty {
            header: hdr,

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _p: Default::default(),
            _q: Default::default(),
            _r: Default::default(),
            _u: Default::default(),
            _v: Default::default(),
            _w: Default::default(),
            _bias_psi: Default::default(),
            _bias_r: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        354
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        354
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default();
        self._p = Default::default();
        self._q = Default::default();
        self._r = Default::default();
        self._u = Default::default();
        self._v = Default::default();
        self._w = Default::default();
        self._bias_psi = Default::default();
        self._bias_r = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        56
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._p);
        bfr.put_f32_le(self._q);
        bfr.put_f32_le(self._r);
        bfr.put_f32_le(self._u);
        bfr.put_f32_le(self._v);
        bfr.put_f32_le(self._w);
        bfr.put_f32_le(self._bias_psi);
        bfr.put_f32_le(self._bias_r);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        self._p = bfr.get_f32_le();
        self._q = bfr.get_f32_le();
        self._r = bfr.get_f32_le();
        self._u = bfr.get_f32_le();
        self._v = bfr.get_f32_le();
        self._w = bfr.get_f32_le();
        self._bias_psi = bfr.get_f32_le();
        self._bias_r = bfr.get_f32_le();

        Ok(())
    }
}
