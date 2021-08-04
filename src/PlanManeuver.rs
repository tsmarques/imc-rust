use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// Named plan maneuver.
#[derive(Default)]
pub struct PlanManeuver {
    /// IMC Header
    pub header: Header,

    /// The maneuver ID.
    pub _maneuver_id: String,

    /// The maneuver specification.
    pub _data: Option<Box<Maneuver>>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan activation.
    pub _start_actions: MessageList<dyn Message>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan termination.
    pub _end_actions: MessageList<dyn Message>,
}

impl PlanManeuver {
    pub fn new() -> PlanManeuver {
        let mut msg = PlanManeuver {
            header: Header::new(552),

            _maneuver_id: Default::default(),
            _data: Default::default(),
            _start_actions: vec![],
            _end_actions: vec![],
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
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }

        for msg in self._end_actions.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._maneuver_id.len() + 2;

        match &self._data {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        for msg in self._start_actions.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        for msg in self._end_actions.iter() {
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
        serialize_bytes!(bfr, self._maneuver_id.as_bytes());
        match &self._data {
            None => {}

            Some(m) => m.serialize_fields(bfr),
        };
        for msg in self._start_actions.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
        for msg in self._end_actions.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }
}
