use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// This message is used to send a periodic update of values for
/// each remote action. If the action is not on the list the assumed
/// value is 0.
#[derive(Default)]
pub struct RemoteActions {
    /// IMC Header
    pub header: Header,

    /// List of values for each remote action (e.g: &quot;Propeller=0.6,PanTilt=0.75,Lights=1&quot;).
    pub _actions: String,
}

impl RemoteActions {
    pub fn new() -> RemoteActions {
        let mut msg = RemoteActions {
            header: Header::new(305),

            _actions: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for RemoteActions {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        305
    }

    fn clear(&mut self) {
        self.header.clear();

        self._actions = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._actions.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._actions.as_bytes());
    }
}
