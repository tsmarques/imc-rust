use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Information regarding a sent/received Sonar pulse.
#[derive(Default)]
pub struct SonarPulse {
    /// IMC Header
    pub header: Header,

    /// Frequency of the sent/received sonar pulse.
    pub _frequency: i32,

    /// Pulse Length of the sonar pulse.
    pub _pulse_length: i32,

    /// Time Delay of the sonar pulse.
    pub _time_delay: i32,

    /// Doppler shift added to the sonar pulse in retransmission
    pub _simulated_speed: i32,
}

impl Message for SonarPulse {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = SonarPulse {
            header: Header::new(2006),

            _frequency: Default::default(),
            _pulse_length: Default::default(),
            _time_delay: Default::default(),
            _simulated_speed: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = SonarPulse {
            header: hdr,

            _frequency: Default::default(),
            _pulse_length: Default::default(),
            _time_delay: Default::default(),
            _simulated_speed: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        2006
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2006
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._frequency = Default::default();
        self._pulse_length = Default::default();
        self._time_delay = Default::default();
        self._simulated_speed = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_i32_le(self._frequency);
        bfr.put_i32_le(self._pulse_length);
        bfr.put_i32_le(self._time_delay);
        bfr.put_i32_le(self._simulated_speed);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._frequency = bfr.get_i32_le();
        self._pulse_length = bfr.get_i32_le();
        self._time_delay = bfr.get_i32_le();
        self._simulated_speed = bfr.get_i32_le();

        Ok(())
    }
}
