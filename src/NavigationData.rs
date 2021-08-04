use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of navigation data.
/// This is constituted by data which is not
/// part of the vehicle estimated state but
/// that the user may refer for more information.
#[derive(Default)]
pub struct NavigationData {
    /// IMC Header
    pub header: Header,

    /// The psi Euler angle bias from the vehicle's sensed attitude.
    pub _bias_psi: f32,

    /// The angular velocity over body-fixed zz axis bias from sensor.
    pub _bias_r: f32,

    /// Course over ground given by NED ground velocity vectors.
    pub _cog: f32,

    /// Continuous psi Euler angle (without normalizations).
    pub _cyaw: f32,

    /// GPS rejection filter level.
    pub _lbl_rej_level: f32,

    /// LBL rejection filter level.
    pub _gps_rej_level: f32,

    /// Custom variable.
    pub _custom_x: f32,

    /// Custom variable.
    pub _custom_y: f32,

    /// Custom variable.
    pub _custom_z: f32,
}

impl NavigationData {
    pub fn new() -> NavigationData {
        let mut msg = NavigationData {
            header: Header::new(355),

            _bias_psi: Default::default(),
            _bias_r: Default::default(),
            _cog: Default::default(),
            _cyaw: Default::default(),
            _lbl_rej_level: Default::default(),
            _gps_rej_level: Default::default(),
            _custom_x: Default::default(),
            _custom_y: Default::default(),
            _custom_z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for NavigationData {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        355
    }

    fn clear(&mut self) {
        self.header.clear();

        self._bias_psi = Default::default();

        self._bias_r = Default::default();

        self._cog = Default::default();

        self._cyaw = Default::default();

        self._lbl_rej_level = Default::default();

        self._gps_rej_level = Default::default();

        self._custom_x = Default::default();

        self._custom_y = Default::default();

        self._custom_z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        36
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._bias_psi);
        bfr.put_f32_le(self._bias_r);
        bfr.put_f32_le(self._cog);
        bfr.put_f32_le(self._cyaw);
        bfr.put_f32_le(self._lbl_rej_level);
        bfr.put_f32_le(self._gps_rej_level);
        bfr.put_f32_le(self._custom_x);
        bfr.put_f32_le(self._custom_y);
        bfr.put_f32_le(self._custom_z);
    }
}
