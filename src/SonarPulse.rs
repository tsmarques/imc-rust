use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Information regarding a sent/received Sonar pulse.
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

impl SonarPulse {
    pub fn new() -> SonarPulse {
        let mut msg = SonarPulse {
            header: Header::new(2006),

            _frequency: Default::default(),
            _pulse_length: Default::default(),
            _time_delay: Default::default(),
            _simulated_speed: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SonarPulse {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        2006
    }

    fn clear(&mut self) {
        self.header.clear();

        self._frequency = Default::default();

        self._pulse_length = Default::default();

        self._time_delay = Default::default();

        self._simulated_speed = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_i32_le(self._frequency);
        bfr.put_i32_le(self._pulse_length);
        bfr.put_i32_le(self._time_delay);
        bfr.put_i32_le(self._simulated_speed);

        serialize_footer(bfr);
    }
}
