use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::BeamConfig::BeamConfig;

use crate::DeviceState::DeviceState;

use crate::packet::ImcError;
use crate::packet::*;

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

impl Message for Distance {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Distance {
            header: Header::new(262),

            _validity: Default::default(),
            _location: vec![],
            _beam_config: vec![],
            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Distance {
            header: hdr,

            _validity: Default::default(),
            _location: vec![],
            _beam_config: vec![],
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        262
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        262
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._validity = Default::default();
        self._location = Default::default();
        self._beam_config = Default::default();
        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        message_list_serialization_size!(dyn_size, self._location);

        message_list_serialization_size!(dyn_size, self._beam_config);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._validity);
        serialize_message_list!(bfr, self._location);
        serialize_message_list!(bfr, self._beam_config);
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._validity = bfr.get_u8();
        self._location = deserialize_message_list_as::<DeviceState>(bfr)?;
        self._beam_config = deserialize_message_list_as::<BeamConfig>(bfr)?;
        self._value = bfr.get_f32_le();

        Ok(())
    }
}
