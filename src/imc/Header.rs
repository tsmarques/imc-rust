use bytes::{BytesMut, BufMut, LittleEndian};
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

pub struct Header
{
    // 
    pub sync :u16,
    // 
    pub mgid :u16,
    // 
    pub size :u16,
    // 
    pub timestamp :f64,
    // 
    pub src :u16,
    // 
    pub src_ent :u8,
    // 
    pub dst :u16,
    // 
    pub dst_ent :u8,
}

impl Header
{
    pub(crate) fn new(msg_id :u16) -> Header
    {
        let mut header = Header {
            sync: 0xFE54,
            mgid: 0,
            size: 0,
            timestamp: 0.0,
            src: 0,
            src_ent: 0,
            dst: 0,
            dst_ent: 0,
        };

        header
    }

    pub fn serialize(&self, bfr :&mut bytes::BytesMut)
    {
        bfr.put_u16_le(self.sync);
        bfr.put_u16_le(self.mgid);
        bfr.put_u16_le(self.size);
        bfr.put_f64_le(self.timestamp);
        bfr.put_u16_le(self.src);
        bfr.put_u8(self.src_ent);
        bfr.put_u16_le(self.dst);
        bfr.put_u8(self.dst_ent);
    }
}