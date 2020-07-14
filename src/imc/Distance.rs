use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

use crate::imc::BeamConfig::BeamConfig;

use crate::imc::DeviceState::DeviceState;

pub enum ValidityEnum {
    // Invalid
    DV_INVALID = 0,
    // Valid
    DV_VALID = 1,
}

impl ValidityEnum {
    pub fn as_u32(&self) -> u32 {
        match self {
            DV_INVALID => 0,
            DV_VALID => 1,
        }
    }
}

/// Measurement is invalid.
pub struct Distance {
    /// IMC Header
    pub header: Header,

    /// Measurement is valid.
    pub _validity: u8,

    /// Device Location in the system.
    pub _location: Vec<Box<DeviceState>>,

    /// Beam configuration of the device.
    pub _beam_config: Vec<Box<BeamConfig>>,

    /// Measured distance.
    pub _value: f32,
}

impl Distance {
    pub fn new() -> Distance {
        let mut msg = Distance {
            header: Header::new(262),

            _validity: Default::default(),
            _location: Default::default(),
            _beam_config: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Distance {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        262
    }

    fn clear(&mut self) {
        self.header.clear();

        self._validity = Default::default();

        for msg in self._location.iter_mut() {
            msg.clear();
        }

        for msg in self._beam_config.iter_mut() {
            msg.clear();
        }

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._validity);
        for msg in self._location.iter() {
            msg.serialize(bfr);
        }
        for msg in self._beam_config.iter() {
            msg.serialize(bfr);
        }
        bfr.put_f32_le(self._value);

        serialize_footer(bfr);
    }
}
