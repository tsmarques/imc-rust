use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for IdleManeuver {}

/// Causes the vehicle to stay idle for some time.
/// message-group: Maneuver
#[derive(Default)]
pub struct IdleManeuver {
    /// IMC Header
    pub header: Header,

    /// Optional duration of the Idle maneuver. Use '0' for unlimited
    /// duration time (maneuver will have to be explicitly stopped).
    pub _duration: u16,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl IdleManeuver {
    pub fn new() -> IdleManeuver {
        let mut msg = IdleManeuver {
            header: Header::new(454),

            _duration: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for IdleManeuver {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        454
    }

    fn clear(&mut self) {
        self.header.clear();

        self._duration = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._duration);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }
}
