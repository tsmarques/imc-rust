use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

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
    pub _actions: MessageList<Box<dyn Message>>,
}

impl Message for PlanTransition {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PlanTransition {
            header: Header::new(553),

            _source_man: Default::default(),
            _dest_man: Default::default(),
            _conditions: Default::default(),
            _actions: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PlanTransition {
            header: hdr,

            _source_man: Default::default(),
            _dest_man: Default::default(),
            _conditions: Default::default(),
            _actions: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        553
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        553
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._source_man = Default::default();

        self._dest_man = Default::default();

        self._conditions = Default::default();

        self._actions = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._source_man.len() + 2;

        dyn_size += self._dest_man.len() + 2;

        dyn_size += self._conditions.len() + 2;

        message_list_serialization_size!(dyn_size, self._actions);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._source_man.as_bytes());
        serialize_bytes!(bfr, self._dest_man.as_bytes());
        serialize_bytes!(bfr, self._conditions.as_bytes());
        serialize_message_list!(bfr, self._actions);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._source_man);

        deserialize_string!(bfr, self._dest_man);

        deserialize_string!(bfr, self._conditions);

        for m in self._actions.iter_mut() {
            m.deserialize_fields(bfr);
        }
    }
}
