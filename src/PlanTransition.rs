use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

/// Describes a plan transition within a plan specification. A
/// transition states the vehicle conditions that must be met to
/// signal the transition, the maneuver that should be started as a
/// result, and an optional set of actions triggered by the
/// transition.
#[derive(Default)]
pub struct PlanTransition {
    /// IMC Header
    pub header: Header,

    /// Comma separated list of maneuver IDs, or the special value '.'
    /// to identify a global plan transition.
    pub _source_man: String,

    /// Target maneuver name.
    /// If it equals the special value '_done_' then plan should
    /// terminate with a success status.
    /// If it equals the special value '_error_' then the plan should
    /// terminate with an error status.
    pub _dest_man: String,

    /// Comma separated list of conditions for transition. Each
    /// condition identifier corresponds to a known predicate which is
    /// interpreted and tested internally by the vehicle.
    pub _conditions: String,

    /// Messages processed when the transition is triggered.
    pub _actions: MessageList<dyn Message>,
}

impl PlanTransition {
    pub fn new() -> PlanTransition {
        let mut msg = PlanTransition {
            header: Header::new(553),

            _source_man: Default::default(),
            _dest_man: Default::default(),
            _conditions: Default::default(),
            _actions: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanTransition {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        553
    }

    fn clear(&mut self) {
        self.header.clear();

        self._source_man = Default::default();

        self._dest_man = Default::default();

        self._conditions = Default::default();

        for msg in self._actions.iter_mut() {
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

        dyn_size += self._source_man.len() + 2;

        dyn_size += self._dest_man.len() + 2;

        dyn_size += self._conditions.len() + 2;

        for msg in self._actions.iter() {
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
        serialize_bytes!(bfr, self._source_man.as_bytes());
        serialize_bytes!(bfr, self._dest_man.as_bytes());
        serialize_bytes!(bfr, self._conditions.as_bytes());
        for msg in self._actions.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }
}
