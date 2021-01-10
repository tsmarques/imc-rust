use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// This message presents the simulated state of the vehicle. The simulated
/// state attempts to provide a realistic state interpretation of operating
/// various kinds of vehicles.
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

impl SimulatedState {
    pub fn new() -> SimulatedState {
        let mut msg = SimulatedState {
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

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SimulatedState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        50
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

    fn fixed_serialization_size(&self) -> usize {
        80
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

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

        serialize_footer(bfr);
    }
}
