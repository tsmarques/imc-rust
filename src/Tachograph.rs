use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// This messages is used to record system activity parameters. These
/// parameters are mainly used for used for maintenance purposes.
pub struct Tachograph {
    /// IMC Header
    pub header: Header,

    /// The time when the last service was performed. The number of
    /// seconds is represented in Universal Coordinated Time (UCT) in
    /// seconds since Jan 1, 1970.
    pub _timestamp_last_service: f64,

    /// Amount of time until the next recommended service.
    pub _time_next_service: f32,

    /// Amount of time the motor can run until the next recommended service.
    pub _time_motor_next_service: f32,

    /// Amount of time the system spent idle on the ground.
    pub _time_idle_ground: f32,

    /// Amount of time the system spent idle in the air.
    pub _time_idle_air: f32,

    /// Amount of time the system spent idle on the water (not submerged).
    pub _time_idle_water: f32,

    /// Amount of time the system spent idle underwater.
    pub _time_idle_underwater: f32,

    /// Amount of time the system spent idle in an unknown medium.
    pub _time_idle_unknown: f32,

    /// Amount of time the system spent on the ground with the motor running.
    pub _time_motor_ground: f32,

    /// Amount of time the system spent in the air with the motor running.
    pub _time_motor_air: f32,

    /// Amount of time the system spent on the water (not submerged) with the motor running.
    pub _time_motor_water: f32,

    /// Amount of time the system spent underwater with the motor running.
    pub _time_motor_underwater: f32,

    /// Amount of time the system spent in an unknown medium with the motor running.
    pub _time_motor_unknown: f32,

    /// The minimum recorded RPM value.
    pub _rpm_min: i16,

    /// The maximum recorded RPM value.
    pub _rpm_max: i16,

    /// The maximum recorded depth value.
    pub _depth_max: f32,
}

impl Tachograph {
    pub fn new() -> Tachograph {
        let mut msg = Tachograph {
            header: Header::new(905),

            _timestamp_last_service: Default::default(),
            _time_next_service: Default::default(),
            _time_motor_next_service: Default::default(),
            _time_idle_ground: Default::default(),
            _time_idle_air: Default::default(),
            _time_idle_water: Default::default(),
            _time_idle_underwater: Default::default(),
            _time_idle_unknown: Default::default(),
            _time_motor_ground: Default::default(),
            _time_motor_air: Default::default(),
            _time_motor_water: Default::default(),
            _time_motor_underwater: Default::default(),
            _time_motor_unknown: Default::default(),
            _rpm_min: Default::default(),
            _rpm_max: Default::default(),
            _depth_max: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Tachograph {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        905
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timestamp_last_service = Default::default();

        self._time_next_service = Default::default();

        self._time_motor_next_service = Default::default();

        self._time_idle_ground = Default::default();

        self._time_idle_air = Default::default();

        self._time_idle_water = Default::default();

        self._time_idle_underwater = Default::default();

        self._time_idle_unknown = Default::default();

        self._time_motor_ground = Default::default();

        self._time_motor_air = Default::default();

        self._time_motor_water = Default::default();

        self._time_motor_underwater = Default::default();

        self._time_motor_unknown = Default::default();

        self._rpm_min = Default::default();

        self._rpm_max = Default::default();

        self._depth_max = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        64
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._timestamp_last_service);
        bfr.put_f32_le(self._time_next_service);
        bfr.put_f32_le(self._time_motor_next_service);
        bfr.put_f32_le(self._time_idle_ground);
        bfr.put_f32_le(self._time_idle_air);
        bfr.put_f32_le(self._time_idle_water);
        bfr.put_f32_le(self._time_idle_underwater);
        bfr.put_f32_le(self._time_idle_unknown);
        bfr.put_f32_le(self._time_motor_ground);
        bfr.put_f32_le(self._time_motor_air);
        bfr.put_f32_le(self._time_motor_water);
        bfr.put_f32_le(self._time_motor_underwater);
        bfr.put_f32_le(self._time_motor_unknown);
        bfr.put_i16_le(self._rpm_min);
        bfr.put_i16_le(self._rpm_max);
        bfr.put_f32_le(self._depth_max);

        serialize_footer(bfr);
    }
}
