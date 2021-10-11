use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum OperationModeEnum {
    // Service
    VS_SERVICE = 0,
    // Calibration
    VS_CALIBRATION = 1,
    // Error
    VS_ERROR = 2,
    // Maneuver
    VS_MANEUVER = 3,
    // External Control
    VS_EXTERNAL = 4,
    // Boot
    VS_BOOT = 5,
}

impl OperationModeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            VS_SERVICE => 0,
            VS_CALIBRATION => 1,
            VS_ERROR => 2,
            VS_MANEUVER => 3,
            VS_EXTERNAL => 4,
            VS_BOOT => 5,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Maneuver Done
    VFLG_MANEUVER_DONE = 0x01,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            VFLG_MANEUVER_DONE => 0x01,
        }
    }
}

/// External control is active.
#[derive(Default)]
pub struct VehicleState {
    /// IMC Header
    pub header: Header,

    /// Booting system.
    pub _op_mode: u8,

    /// Error count for monitored entitites.
    pub _error_count: u8,

    /// The monitored entities with error conditions. This is a comma
    /// separated list of entity names.
    pub _error_ents: String,

    /// Type of maneuver being executed, when in MANEUVER mode. The
    /// value is the IMC serialization ID of the corresponding
    /// maneuver.
    pub _maneuver_type: u16,

    /// Start time of maneuver being executed (Epoch time), when in
    /// MANEUVER mode.
    pub _maneuver_stime: f64,

    /// Estimated time for maneuver completion. The value will be
    /// 65535 if the time is unknown or undefined.
    pub _maneuver_eta: u16,

    /// Enabled control loops.
    pub _control_loops: u32,

    pub _flags: u8,

    /// Description of last error.
    pub _last_error: String,

    /// Time of last error (Epoch time).
    pub _last_error_time: f64,
}

impl Message for VehicleState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = VehicleState {
            header: Header::new(500),

            _op_mode: Default::default(),
            _error_count: Default::default(),
            _error_ents: Default::default(),
            _maneuver_type: Default::default(),
            _maneuver_stime: Default::default(),
            _maneuver_eta: Default::default(),
            _control_loops: Default::default(),
            _flags: Default::default(),
            _last_error: Default::default(),
            _last_error_time: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = VehicleState {
            header: hdr,

            _op_mode: Default::default(),
            _error_count: Default::default(),
            _error_ents: Default::default(),
            _maneuver_type: Default::default(),
            _maneuver_stime: Default::default(),
            _maneuver_eta: Default::default(),
            _control_loops: Default::default(),
            _flags: Default::default(),
            _last_error: Default::default(),
            _last_error_time: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        500
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        500
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op_mode = Default::default();

        self._error_count = Default::default();

        self._error_ents = Default::default();

        self._maneuver_type = Default::default();

        self._maneuver_stime = Default::default();

        self._maneuver_eta = Default::default();

        self._control_loops = Default::default();

        self._flags = Default::default();

        self._last_error = Default::default();

        self._last_error_time = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        27
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._error_ents.len() + 2;

        dyn_size += self._last_error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op_mode);
        bfr.put_u8(self._error_count);
        serialize_bytes!(bfr, self._error_ents.as_bytes());
        bfr.put_u16_le(self._maneuver_type);
        bfr.put_f64_le(self._maneuver_stime);
        bfr.put_u16_le(self._maneuver_eta);
        bfr.put_u32_le(self._control_loops);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._last_error.as_bytes());
        bfr.put_f64_le(self._last_error_time);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op_mode = bfr.get_u8();

        self._error_count = bfr.get_u8();

        deserialize_string!(bfr, self._error_ents);

        self._maneuver_type = bfr.get_u16_le();

        self._maneuver_stime = bfr.get_f64_le();

        self._maneuver_eta = bfr.get_u16_le();

        self._control_loops = bfr.get_u32_le();

        self._flags = bfr.get_u8();

        deserialize_string!(bfr, self._last_error);

        self._last_error_time = bfr.get_f64_le();

        Ok(())
    }
}
