use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for CustomManeuver {}

/// The Custom Maneuver message may be used as specification of a
/// very specific maneuver not covered by the IMC scope. The
/// settings of the maneuver are just its id, timeout and other
/// settings encoded as a tuple list.
/// message-group: Maneuver
pub struct CustomManeuver {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run. If the
    /// maneuver is not completed in the amount of time specified an
    /// error will be generated.
    pub _timeout: u16,

    /// The maneuver name, used as key by an implementation to bind
    /// the maneuver to the corresponding controller.
    pub _name: String,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl CustomManeuver {
    pub fn new() -> CustomManeuver {
        let mut msg = CustomManeuver {
            header: Header::new(465),

            _timeout: Default::default(),
            _name: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for CustomManeuver {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        465
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeout = Default::default();

        self._name = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._custom.as_bytes());

        serialize_footer(bfr);
    }
}
