use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::PlanVariable::PlanVariable;

use crate::PlanManeuver::PlanManeuver;

use crate::PlanTransition::PlanTransition;

/// Identity and description of a plan's general parameters,
/// associated with plan loading (i.e. load plan command in
/// *PlanCommand*).
///  
/// A plan specification is defined by a plan identifier, a set of
/// maneuver specifications and a start maneuver from that set.
///  
/// See the :ref:`PlanManeuver` message for details on maneuver
/// specification.
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
    pub _variables: Vec<Box<PlanVariable>>,

    /// Indicates the id of the starting maneuver for this plan.
    pub _start_man_id: String,

    /// List of maneuver specifications.
    pub _maneuvers: Vec<Box<PlanManeuver>>,

    /// List of maneuver specifications.
    pub _transitions: Vec<Box<PlanTransition>>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan activation.
    pub _start_actions: Vec<Box<dyn Message>>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan termination.
    pub _end_actions: Vec<Box<dyn Message>>,
}

impl PlanSpecification {
    pub fn new() -> PlanSpecification {
        let mut msg = PlanSpecification {
            header: Header::new(551),

            _plan_id: Default::default(),
            _description: Default::default(),
            _vnamespace: Default::default(),
            _variables: Default::default(),
            _start_man_id: Default::default(),
            _maneuvers: Default::default(),
            _transitions: Default::default(),
            _start_actions: Default::default(),
            _end_actions: Default::default(),
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
            msg.clear();
        }

        self._start_man_id = Default::default();

        for msg in self._maneuvers.iter_mut() {
            msg.clear();
        }

        for msg in self._transitions.iter_mut() {
            msg.clear();
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
        let mut dyn_size: usize = 0;

        dyn_size += self._plan_id.len() + 2;

        dyn_size += self._description.len() + 2;

        dyn_size += self._vnamespace.len() + 2;

        for msg in &self._variables {
            dyn_size += msg.dynamic_serialization_size();
        }

        dyn_size += self._start_man_id.len() + 2;

        for msg in &self._maneuvers {
            dyn_size += msg.dynamic_serialization_size();
        }

        for msg in &self._transitions {
            dyn_size += msg.dynamic_serialization_size();
        }

        for msg in &self._start_actions {
            dyn_size += msg.dynamic_serialization_size();
        }

        for msg in &self._end_actions {
            dyn_size += msg.dynamic_serialization_size();
        }

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._plan_id.as_bytes());
        serialize_bytes!(bfr, self._description.as_bytes());
        serialize_bytes!(bfr, self._vnamespace.as_bytes());
        for msg in self._variables.iter() {
            msg.serialize(bfr);
        }
        serialize_bytes!(bfr, self._start_man_id.as_bytes());
        for msg in self._maneuvers.iter() {
            msg.serialize(bfr);
        }
        for msg in self._transitions.iter() {
            msg.serialize(bfr);
        }
        for msg in self._start_actions.iter() {
            msg.serialize(bfr);
        }
        for msg in self._end_actions.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
