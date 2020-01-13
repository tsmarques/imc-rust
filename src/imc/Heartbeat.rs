use crate::imc::Message::{Message, serialize_footer};
use crate::imc::Header::Header;
use crate::imc::{IMC_CONST_UNK_EID, DUNE_IMC_CONST_SYNC};

use bytes::BufMut;

const c_msg_id :u16 = 150;

struct Heartbeat
{
    header: Header,
}

impl Heartbeat
{
    fn new() -> Heartbeat
    {
        let msg = Heartbeat{
            header :Header{
                sync: DUNE_IMC_CONST_SYNC,
                mgid: c_msg_id,
                size: 0,
                timestamp: 0.0,
                src: 0xffff,
                src_ent: IMC_CONST_UNK_EID,
                dst: 0xffff,
                dst_ent: IMC_CONST_UNK_EID
            }
        };

        msg
    }
}

impl Message for Heartbeat
{
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16
    {
        c_msg_id
    }

    fn clear(&mut self)
    { }

    fn fixed_serialization_size(&self) -> usize
    {
        0
    }

    fn dynamic_serialization_size(&self) -> usize
    {
        0
    }

    fn serialize(&self, bfr :&mut bytes::BytesMut)
    {
        self.header.serialize(bfr);
        serialize_footer(bfr);
    }
}