use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

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
    pub _start_actions: MessageList<Box<dyn Message>>,

    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan termination.
    pub _end_actions: MessageList<Box<dyn Message>>,
}

impl Message for PlanManeuver {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PlanManeuver {
            header: Header::new(552),

            _maneuver_id: Default::default(),
            _data: Default::default(),
            _start_actions: vec![],
            _end_actions: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PlanManeuver {
            header: hdr,

            _maneuver_id: Default::default(),
            _data: Default::default(),
            _start_actions: vec![],
            _end_actions: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        552
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        552
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._maneuver_id = Default::default();

        self._data = Default::default();

        self._start_actions = Default::default();

        self._end_actions = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._maneuver_id.len() + 2;

        inline_message_serialization_size!(dyn_size, self._data);

        message_list_serialization_size!(dyn_size, self._start_actions);

        message_list_serialization_size!(dyn_size, self._end_actions);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._maneuver_id.as_bytes());
        serialize_inline_message!(bfr, self._data);
        serialize_message_list!(bfr, self._start_actions);
        serialize_message_list!(bfr, self._end_actions);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._maneuver_id);

        match &mut self._data {
            None => {}

            Some(m) => m.deserialize_fields(bfr),
        };

        for m in self._start_actions.iter_mut() {
            m.deserialize_fields(bfr);
        }

        for m in self._end_actions.iter_mut() {
            m.deserialize_fields(bfr);
        }
    }
}
