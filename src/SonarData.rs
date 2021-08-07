use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::BeamConfig::BeamConfig;

#[allow(non_camel_case_types)]
pub enum TypeEnum {
    // Sidescan
    ST_SIDESCAN = 0,
    // Echo Sounder
    ST_ECHOSOUNDER = 1,
    // Multibeam
    ST_MULTIBEAM = 2,
}

impl TypeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            ST_SIDESCAN => 0,
            ST_ECHOSOUNDER => 1,
            ST_MULTIBEAM => 2,
        }
    }
}

/// This message contains the data acquired by a single sonar
/// measurement.
#[derive(Default)]
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
    pub _beam_config: MessageList<BeamConfig>,

    /// Data acquired by the measurement.
    pub _data: Vec<u8>,
}

impl Message for SonarData {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = SonarData {
            header: hdr,

            _type: Default::default(),
            _frequency: Default::default(),
            _min_range: Default::default(),
            _max_range: Default::default(),
            _bits_per_point: Default::default(),
            _scale_factor: Default::default(),
            _beam_config: vec![],
            _data: Default::default(),
        };

        msg.get_header()._mgid = 276;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = SonarData {
            header: Header::new(276),

            _type: Default::default(),
            _frequency: Default::default(),
            _min_range: Default::default(),
            _max_range: Default::default(),
            _bits_per_point: Default::default(),
            _scale_factor: Default::default(),
            _beam_config: vec![],
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        276
    }

    fn id(&self) -> u16 {
        276
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        14
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in self._beam_config.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u32_le(self._frequency);
        bfr.put_u16_le(self._min_range);
        bfr.put_u16_le(self._max_range);
        bfr.put_u8(self._bits_per_point);
        bfr.put_f32_le(self._scale_factor);
        for msg in self._beam_config.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
