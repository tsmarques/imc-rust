#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::PlanManeuver::PlanManeuver;

use crate::PlanTransition::PlanTransition;

use crate::PlanVariable::PlanVariable;

/// Identity and description of a plan's general parameters,
/// associated with plan loading (i.e. load plan command in
/// *PlanCommand*).
///  
/// A plan specification is defined by a plan identifier, a set of
/// maneuver specifications and a start maneuver from that set.
///  
/// See the :ref:`PlanManeuver` message for details on maneuver
/// specification.
#[derive(Default)]
pub struct PlanSpecification {
    /// IMC Header
    pub header: Header,

    /// The plan's identifier.
    pub _plan_id: String,

    /// Verbose text description of plan.
    pub _description: String,

    /// Namespace for plan variables.
    pub _vnamespace: String,

    /// Plan variables.
    pub _variables: MessageList<PlanVariable>,

    /// Indicates the id of the starting maneuver for this plan.
    pub _start_man_id: String,

    /// List of maneuver specifications.
    pub _maneuvers: MessageList<PlanManeuver>,

    /// List of maneuver specifications.
    pub _transitions: MessageList<PlanTransition>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan activation.
    pub _start_actions: MessageList<dyn Message>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan termination.
    pub _end_actions: MessageList<dyn Message>,
}

impl PlanSpecification {
    pub fn new() -> PlanSpecification {
        let mut msg = PlanSpecification {
            header: Header::new(551),

            _plan_id: Default::default(),
            _description: Default::default(),
            _vnamespace: Default::default(),
            _variables: vec![],
            _start_man_id: Default::default(),
            _maneuvers: vec![],
            _transitions: vec![],
            _start_actions: vec![],
            _end_actions: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanSpecification {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        551
    }

    fn clear(&mut self) {
        self.header.clear();

        self._plan_id = Default::default();

        self._description = Default::default();

        self._vnamespace = Default::default();

        for msg in self._variables.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }

        self._start_man_id = Default::default();

        for msg in self._maneuvers.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }

        for msg in self._transitions.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
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

        dyn_size += self._plan_id.len() + 2;

        dyn_size += self._description.len() + 2;

        dyn_size += self._vnamespace.len() + 2;

        for msg in self._variables.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        dyn_size += self._start_man_id.len() + 2;

        for msg in self._maneuvers.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        for msg in self._transitions.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
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
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        serialize_bytes!(bfr, self._description.as_bytes());
        serialize_bytes!(bfr, self._vnamespace.as_bytes());
        for msg in self._variables.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
        serialize_bytes!(bfr, self._start_man_id.as_bytes());
        for msg in self._maneuvers.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
        for msg in self._transitions.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
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
