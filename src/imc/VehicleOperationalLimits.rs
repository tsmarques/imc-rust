use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum ActiononthevehicleoperationallimitsEnum {
    // Request
    OP_REQUEST = 0,
    // Set
    OP_SET = 1,
    // Report
    OP_REPORT = 2,
}

impl ActiononthevehicleoperationallimitsEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            OP_REQUEST => 0,
            OP_SET => 1,
            OP_REPORT => 2,
        }
    }
}

/// Vehicle opertional limits.
/// For aircraft this should represent the flight envelope and the dynamic contraints.
pub struct VehicleOperationalLimits {
    /// IMC Header
    pub header: Header,

    /// Action on the vehicle operation limits
    pub _op: u8,

    /// Minimum operation speed.
    /// For aircraft this is equal or larger then the stall speed.
    pub _speed_min: f32,

    /// Maximum operation speed.
    /// For aircraft this is limited by the engine power or structural contrains.
    pub _speed_max: f32,

    /// Maximum longitudinal acceleration.
    pub _long_accel: f32,

    /// Maximum altitude above mean-sea-level.
    pub _alt_max_msl: f32,

    /// Maximum dive rate (negative vertical speed) as a fraction of the longitudinal speed.
    pub _dive_fraction_max: f32,

    /// Maximum climb rate (positive vertical speed) as a fraction of the longitudinal speed.
    pub _climb_fraction_max: f32,

    /// Limit to the bank angle (roll; angle over the xx body-axis).
    pub _bank_max: f32,

    /// Limit to the bank angular rate (roll; angle over the xx body-axis).
    pub _p_max: f32,

    /// Minimum pitch angle (angle over the xx body-axis).
    pub _pitch_min: f32,

    /// Maximum pitch angle (angle over the xx body-axis).
    pub _pitch_max: f32,

    /// Maximum pitch angular rate (angle over the xx body-axis).
    pub _q_max: f32,

    /// Minimum load factor, i.e., maximum positive acceleration in the zz body-axis
    /// as a factor of the gravity acceleration at mean-sea-level.
    pub _g_min: f32,

    /// Maximum load factor, i.e., maximum negative acceleration in the zz body-axis
    /// as a factor of the gravity acceleration at mean-sea-level.
    pub _g_max: f32,

    /// Maximum lateral load factor, i.e., maximum acceleration in the yy body-axis
    /// as a factor of the gravity acceleration at mean-sea-level.
    pub _g_lat_max: f32,

    /// Minimum motor RPMs.
    pub _rpm_min: f32,

    /// Maximum motor RPMs.
    pub _rpm_max: f32,

    /// Maximum motor RPMs' rate of change.
    pub _rpm_rate_max: f32,
}

impl VehicleOperationalLimits {
    pub fn new() -> VehicleOperationalLimits {
        let mut msg = VehicleOperationalLimits {
            header: Header::new(16),

            _op: Default::default(),
            _speed_min: Default::default(),
            _speed_max: Default::default(),
            _long_accel: Default::default(),
            _alt_max_msl: Default::default(),
            _dive_fraction_max: Default::default(),
            _climb_fraction_max: Default::default(),
            _bank_max: Default::default(),
            _p_max: Default::default(),
            _pitch_min: Default::default(),
            _pitch_max: Default::default(),
            _q_max: Default::default(),
            _g_min: Default::default(),
            _g_max: Default::default(),
            _g_lat_max: Default::default(),
            _rpm_min: Default::default(),
            _rpm_max: Default::default(),
            _rpm_rate_max: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for VehicleOperationalLimits {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        16
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._speed_min = Default::default();

        self._speed_max = Default::default();

        self._long_accel = Default::default();

        self._alt_max_msl = Default::default();

        self._dive_fraction_max = Default::default();

        self._climb_fraction_max = Default::default();

        self._bank_max = Default::default();

        self._p_max = Default::default();

        self._pitch_min = Default::default();

        self._pitch_max = Default::default();

        self._q_max = Default::default();

        self._g_min = Default::default();

        self._g_max = Default::default();

        self._g_lat_max = Default::default();

        self._rpm_min = Default::default();

        self._rpm_max = Default::default();

        self._rpm_rate_max = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        69
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        bfr.put_f32_le(self._speed_min);
        bfr.put_f32_le(self._speed_max);
        bfr.put_f32_le(self._long_accel);
        bfr.put_f32_le(self._alt_max_msl);
        bfr.put_f32_le(self._dive_fraction_max);
        bfr.put_f32_le(self._climb_fraction_max);
        bfr.put_f32_le(self._bank_max);
        bfr.put_f32_le(self._p_max);
        bfr.put_f32_le(self._pitch_min);
        bfr.put_f32_le(self._pitch_max);
        bfr.put_f32_le(self._q_max);
        bfr.put_f32_le(self._g_min);
        bfr.put_f32_le(self._g_max);
        bfr.put_f32_le(self._g_lat_max);
        bfr.put_f32_le(self._rpm_min);
        bfr.put_f32_le(self._rpm_max);
        bfr.put_f32_le(self._rpm_rate_max);

        serialize_footer(bfr);
    }
}
