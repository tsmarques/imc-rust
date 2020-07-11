use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 305;

/// This message is used to send a periodic update of values for
/// each remote action. If the action is not on the list the assumed
/// value is 0.
pub struct RemoteActions {
    /// IMC Header
    pub header: Header,

    /// List of values for each remote action (e.g: &quot;Propeller=0.6,PanTilt=0.75,Lights=1&quot;).
    pub _actions: String,
}

impl RemoteActions {
    pub fn new() -> RemoteActions {
        let mut msg = RemoteActions {
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._actions = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._actions.as_bytes());

        serialize_footer(bfr);
    }
}
