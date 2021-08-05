use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum operationEnum {
    // Report
    OP_REPORT = 0,
    // Query
    OP_QUERY = 1,
}

impl operationEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            OP_REPORT => 0,
            OP_QUERY => 1,
        }
    }
}

/// This message is used as query to request for the possible remote
/// actions (operation=QUERY and the list is empty in this
/// case). The vehicle responds using the same message type
/// returning the tuplelist with the pairs: Action,Type
/// (operation=REPORT). The type of action can be Axis, Hat or
/// Button.
#[derive(Default)]
pub struct RemoteActionsRequest {
    /// IMC Header
    pub header: Header,

    /// Operation to perform.
    pub _op: u8,

    /// Example: &quot;Propulsion=Axis,PanTilt=Hat,Lights=Button&quot;
    pub _actions: String,
}

impl RemoteActionsRequest {
    pub fn new() -> RemoteActionsRequest {
        let mut msg = RemoteActionsRequest {
            header: Header::new(304),

            _op: Default::default(),
            _actions: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for RemoteActionsRequest {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        304
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._actions = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._actions.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._actions.as_bytes());
    }
}
