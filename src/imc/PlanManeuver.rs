use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

use crate::imc::MessageGroup::Maneuver;

/// Named plan maneuver.
pub struct PlanManeuver {
    /// IMC Header
    pub header: Header,

    /// The maneuver ID.
    pub _maneuver_id: String,

    /// The maneuver specification.
    pub _data: Option<Box<Maneuver>>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan activation.
    pub _start_actions: Vec<Box<dyn Message>>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan termination.
    pub _end_actions: Vec<Box<dyn Message>>,
}

impl PlanManeuver {
    pub fn new() -> PlanManeuver {
        let mut msg = PlanManeuver {
            header: Header::new(552),

            _maneuver_id: Default::default(),
            _data: Default::default(),
            _start_actions: Default::default(),
            _end_actions: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanManeuver {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        552
    }

    fn clear(&mut self) {
        self.header.clear();

        self._maneuver_id = Default::default();

        match &mut self._data {
            Some(field) => field.clear(),

            None => {}
        }

        for msg in self._start_actions.iter_mut() {
            msg.clear();
        }

        for msg in self._end_actions.iter_mut() {
            msg.clear();
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._maneuver_id.as_bytes());
        match &self._data {
            Some(field) => field.serialize(bfr),

            None => {}
        };
        for msg in self._start_actions.iter() {
            msg.serialize(bfr);
        }
        for msg in self._end_actions.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
