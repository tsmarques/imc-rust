use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum ValidityEnum {
    // Valid Date
    GFV_VALID_DATE = 0x0001,
    // Valid Time
    GFV_VALID_TIME = 0x0002,
    // Valid Position
    GFV_VALID_POS = 0x0004,
    // Valid Course Over Ground
    GFV_VALID_COG = 0x0008,
    // Valid Speed Over Ground
    GFV_VALID_SOG = 0x0010,
    // Valid Horizontal Accuracy Estimate
    GFV_VALID_HACC = 0x0020,
    // Valid Vertical Accuracy Estimate
    GFV_VALID_VACC = 0x0040,
    // Valid Horizontal Dilution of Precision
    GFV_VALID_HDOP = 0x0080,
    // Valid Vertical Dilution of Precision
    GFV_VALID_VDOP = 0x0100,
}

impl ValidityEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            GFV_VALID_DATE => 0x0001,
            GFV_VALID_TIME => 0x0002,
            GFV_VALID_POS => 0x0004,
            GFV_VALID_COG => 0x0008,
            GFV_VALID_SOG => 0x0010,
            GFV_VALID_HACC => 0x0020,
            GFV_VALID_VACC => 0x0040,
            GFV_VALID_HDOP => 0x0080,
            GFV_VALID_VDOP => 0x0100,
        }
    }
}

pub enum TypeEnum {
    // Stand Alone
    GFT_STANDALONE = 0x00,
    // Differential
    GFT_DIFFERENTIAL = 0x01,
    // Dead Reckoning
    GFT_DEAD_RECKONING = 0x02,
    // Manual Input
    GFT_MANUAL_INPUT = 0x03,
    // Simulation
    GFT_SIMULATION = 0x04,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            GFT_STANDALONE => 0x00,
            GFT_DIFFERENTIAL => 0x01,
            GFT_DEAD_RECKONING => 0x02,
            GFT_MANUAL_INPUT => 0x03,
            GFT_SIMULATION => 0x04,
        }
    }
}

/// Simulated solution.
pub struct GpsFix {
    /// IMC Header
    pub header: Header,

    /// Field 'hdop' is valid.
    pub _validity: u16,

    /// Manual solution.
    pub _type: u8,

    /// UTC year.
    pub _utc_year: u16,

    /// UTC month.
    pub _utc_month: u8,

    /// UTC day.
    pub _utc_day: u8,

    /// UTC time of the GPS fix measured in seconds since 00:00:00 (midnight).
    pub _utc_time: f32,

    /// WGS-84 Latitude coordinate.
    pub _lat: f64,

    /// WGS-84 Longitude coordinate.
    pub _lon: f64,

    /// Height above WGS-84 ellipsoid.
    pub _height: f32,

    /// Number of satellites used by the GPS device to compute the
    /// solution.
    pub _satellites: u8,

    /// Course Over Ground (true).
    pub _cog: f32,

    /// Speed Over Ground.
    pub _sog: f32,

    /// Horizontal dilution of precision.
    pub _hdop: f32,

    /// Vertical dilution of precision.
    pub _vdop: f32,

    /// Horizontal Accuracy Estimate.
    pub _hacc: f32,

    /// Vertical Accuracy Estimate.
    pub _vacc: f32,
}

impl GpsFix {
    pub fn new() -> GpsFix {
        let mut msg = GpsFix {
            header: Header::new(253),

            _validity: Default::default(),
            _type: Default::default(),
            _utc_year: Default::default(),
            _utc_month: Default::default(),
            _utc_day: Default::default(),
            _utc_time: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _satellites: Default::default(),
            _cog: Default::default(),
            _sog: Default::default(),
            _hdop: Default::default(),
            _vdop: Default::default(),
            _hacc: Default::default(),
            _vacc: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for GpsFix {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        253
    }

    fn clear(&mut self) {
        self.header.clear();

        self._validity = Default::default();

        self._type = Default::default();

        self._utc_year = Default::default();

        self._utc_month = Default::default();

        self._utc_day = Default::default();

        self._utc_time = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._height = Default::default();

        self._satellites = Default::default();

        self._cog = Default::default();

        self._sog = Default::default();

        self._hdop = Default::default();

        self._vdop = Default::default();

        self._hacc = Default::default();

        self._vacc = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        56
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u16_le(self._validity);
        bfr.put_u8(self._type);
        bfr.put_u16_le(self._utc_year);
        bfr.put_u8(self._utc_month);
        bfr.put_u8(self._utc_day);
        bfr.put_f32_le(self._utc_time);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        bfr.put_u8(self._satellites);
        bfr.put_f32_le(self._cog);
        bfr.put_f32_le(self._sog);
        bfr.put_f32_le(self._hdop);
        bfr.put_f32_le(self._vdop);
        bfr.put_f32_le(self._hacc);
        bfr.put_f32_le(self._vacc);

        serialize_footer(bfr);
    }
}
