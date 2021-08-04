use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Entity is Inactive
    EAS_INACTIVE = 0,
    // Entity is Active
    EAS_ACTIVE = 1,
    // Activation in Progress
    EAS_ACT_IP = 2,
    // Activation Completed
    EAS_ACT_DONE = 3,
    // Activation Failed
    EAS_ACT_FAIL = 4,
    // Deactivation In Progress
    EAS_DEACT_IP = 5,
    // Deactivation Completed
    EAS_DEACT_DONE = 6,
    // Deactivation Failed
    EAS_DEACT_FAIL = 7,
}

impl StateEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            EAS_INACTIVE => 0,
            EAS_ACTIVE => 1,
            EAS_ACT_IP => 2,
            EAS_ACT_DONE => 3,
            EAS_ACT_FAIL => 4,
            EAS_DEACT_IP => 5,
            EAS_DEACT_DONE => 6,
            EAS_DEACT_FAIL => 7,
        }
    }
}

/// Deactivation is in progress.
#[derive(Default)]
pub struct EntityActivationState {
    /// IMC Header
    pub header: Header,

    /// The deactivation procedure failed and the field 'error' contains the error message.
    pub _state: u8,

    /// Human-readable error message.
    pub _error: String,
}

impl EntityActivationState {
    pub fn new() -> EntityActivationState {
        let mut msg = EntityActivationState {
            header: Header::new(14),

            _state: Default::default(),
            _error: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EntityActivationState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        14
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();

        self._error = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._error.as_bytes());
    }
}
