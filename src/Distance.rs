use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::BeamConfig::BeamConfig;

use crate::DeviceState::DeviceState;

#[allow(non_camel_case_types)]
pub enum ValidityEnum {
    // Invalid
    DV_INVALID = 0,
    // Valid
    DV_VALID = 1,
}

impl ValidityEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            DV_INVALID => 0,
            DV_VALID => 1,
        }
    }
}

/// Measurement is invalid.
#[derive(Default)]
pub struct Distance {
    /// IMC Header
    pub header: Header,

    /// Measurement is valid.
    pub _validity: u8,

    /// Device Location in the system.
    pub _location: MessageList<DeviceState>,

    /// Beam configuration of the device.
    pub _beam_config: MessageList<BeamConfig>,

    /// Measured distance.
    pub _value: f32,
}

impl Distance {
    pub fn new() -> Distance {
        let mut msg = Distance {
            header: Header::new(262),

            _validity: Default::default(),
            _location: vec![],
            _beam_config: vec![],
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
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }

        for msg in self._beam_config.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in self._location.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        for msg in self._beam_config.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._validity);
        for msg in self._location.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
        for msg in self._beam_config.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
        bfr.put_f32_le(self._value);
    }
}
