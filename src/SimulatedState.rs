use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// This message presents the simulated state of the vehicle. The simulated
/// state attempts to provide a realistic state interpretation of operating
/// various kinds of vehicles.
#[derive(Default)]
pub struct SimulatedState {
    /// IMC Header
    pub header: Header,

    /// WGS-84 Latitude.
    pub _lat: f64,

    /// WGS-84 Longitude.
    pub _lon: f64,

    /// Height above the WGS-84 ellipsoid.
    pub _height: f32,

    /// The North offset of the North/East/Down field.
    pub _x: f32,

    /// The East offset of the North/East/Down field.
    pub _y: f32,

    /// The Down offset of the North/East/Down field.
    pub _z: f32,

    /// The phi Euler angle from the vehicle's attitude.
    pub _phi: f32,

    /// The theta Euler angle from the vehicle's attitude.
    pub _theta: f32,

    /// The psi Euler angle from the vehicle's attitude.
    pub _psi: f32,

    /// Body-fixed frame xx axis linear velocity component.
    pub _u: f32,

    /// Body-fixed frame yy axis linear velocity component.
    pub _v: f32,

    /// Body-fixed frame zz axis linear velocity component.
    pub _w: f32,

    /// The angular velocity over body-fixed xx axis (roll rate).
    pub _p: f32,

    /// The angular velocity over body-fixed yy axis (pitch rate).
    pub _q: f32,

    /// The angular velocity over body-fixed zz axis (yaw rate).
    pub _r: f32,

    /// Stream Velocity xx axis velocity component.
    pub _svx: f32,

    /// Stream Velocity yy axis velocity component.
    pub _svy: f32,

    /// Stream Velocity zz axis velocity component.
    pub _svz: f32,
}

impl Message for SimulatedState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = SimulatedState {
            header: Header::new(50),

            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _u: Default::default(),
            _v: Default::default(),
            _w: Default::default(),
            _p: Default::default(),
            _q: Default::default(),
            _r: Default::default(),
            _svx: Default::default(),
            _svy: Default::default(),
            _svz: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = SimulatedState {
            header: hdr,

            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _u: Default::default(),
            _v: Default::default(),
            _w: Default::default(),
            _p: Default::default(),
            _q: Default::default(),
            _r: Default::default(),
            _svx: Default::default(),
            _svy: Default::default(),
            _svz: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        50
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        50
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._lat = Default::default();
        self._lon = Default::default();
        self._height = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default();
        self._u = Default::default();
        self._v = Default::default();
        self._w = Default::default();
        self._p = Default::default();
        self._q = Default::default();
        self._r = Default::default();
        self._svx = Default::default();
        self._svy = Default::default();
        self._svz = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        80
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._u);
        bfr.put_f32_le(self._v);
        bfr.put_f32_le(self._w);
        bfr.put_f32_le(self._p);
        bfr.put_f32_le(self._q);
        bfr.put_f32_le(self._r);
        bfr.put_f32_le(self._svx);
        bfr.put_f32_le(self._svy);
        bfr.put_f32_le(self._svz);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._height = bfr.get_f32_le();
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        self._u = bfr.get_f32_le();
        self._v = bfr.get_f32_le();
        self._w = bfr.get_f32_le();
        self._p = bfr.get_f32_le();
        self._q = bfr.get_f32_le();
        self._r = bfr.get_f32_le();
        self._svx = bfr.get_f32_le();
        self._svy = bfr.get_f32_le();
        self._svz = bfr.get_f32_le();

        Ok(())
    }
}
