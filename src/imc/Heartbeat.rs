use crate::imc::Message::*;
use crate::imc::{IMC_CONST_UNK_EID, DUNE_IMC_CONST_SYNC};

use bytes::BufMut;
use crate::imc::Header::Header;


const c_msg_id :u16 = 150;






/// The Heartbeat message is used to inform other modules that the 
/// sending entity's system is running normally and communications 
/// are alive. 
pub struct Heartbeat {
    /// IMC Header
    pub header: Header,

}

impl Heartbeat {
    pub fn new() -> Heartbeat {
        let mut msg = Heartbeat {
            header : Header::new(c_msg_id),

        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Heartbeat {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();
        
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr :&mut bytes::BytesMut) {
        self.header.serialize(bfr);


        serialize_footer(bfr);
    }
}