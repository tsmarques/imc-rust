use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::BeamConfig::BeamConfig;

pub enum TypeEnum {
    // Sidescan
    ST_SIDESCAN = 0,
    // Echo Sounder
    ST_ECHOSOUNDER = 1,
    // Multibeam
    ST_MULTIBEAM = 2,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            ST_SIDESCAN => 0,
            ST_ECHOSOUNDER => 1,
            ST_MULTIBEAM => 2,
        }
    }
}

/// This message contains the data acquired by a single sonar
/// measurement.
pub struct SonarData {
    /// IMC Header
    pub header: Header,

    /// Type of sonar.
    pub _type: u8,

    /// Operating frequency.
    pub _frequency: u32,

    /// Minimum range.
    pub _min_range: u16,

    /// Maximum range.
    pub _max_range: u16,

    /// Size of the data unit.
    pub _bits_per_point: u8,

    /// Scaling factor used to multiply each data unit to restore the
    /// original floating point value.
    pub _scale_factor: f32,

    /// Beam configuration of the device.
    pub _beam_config: Vec<Box<BeamConfig>>,

    /// Data acquired by the measurement.
    pub _data: Vec<u8>,
}

impl SonarData {
    pub fn new() -> SonarData {
        let mut msg = SonarData {
            header: Header::new(276),

            _type: Default::default(),
            _frequency: Default::default(),
            _min_range: Default::default(),
            _max_range: Default::default(),
            _bits_per_point: Default::default(),
            _scale_factor: Default::default(),
            _beam_config: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SonarData {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        276
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._frequency = Default::default();

        self._min_range = Default::default();

        self._max_range = Default::default();

        self._bits_per_point = Default::default();

        self._scale_factor = Default::default();

        for msg in self._beam_config.iter_mut() {
            msg.clear();
        }

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        14
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in &self._beam_config {
            dyn_size += msg.dynamic_serialization_size();
        }

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._type);
        bfr.put_u32_le(self._frequency);
        bfr.put_u16_le(self._min_range);
        bfr.put_u16_le(self._max_range);
        bfr.put_u8(self._bits_per_point);
        bfr.put_f32_le(self._scale_factor);
        for msg in self._beam_config.iter() {
            msg.serialize(bfr);
        }
        serialize_bytes!(bfr, self._data.as_slice());

        serialize_footer(bfr);
    }
}
