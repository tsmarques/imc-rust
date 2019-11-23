use bytes::{BytesMut, BufMut, LittleEndian};
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

pub struct Header
{
    // Syncronization number
    pub sync: u16,
    // Message Id
    pub mgid: u16,
    // Message size
    pub size: u16,
    // Time stamp
    pub timestamp: f64,
    // Source address
    pub src: u16,
    // Source entity
    pub src_ent: u8,
    // Destination Address
    pub dst: u16,
    // Destination Entity
    pub dst_ent: u8,
}

impl Header
{
    pub(crate) fn new(msg_id :u16) -> Header
    {
        let mut header = Header {
            sync: DUNE_IMC_CONST_SYNC,
            mgid: msg_id,
            size: 0,
            timestamp: 0.0,
            src: 0xffff,
            src_ent: IMC_CONST_UNK_EID,
            dst: 0xffff,
            dst_ent: IMC_CONST_UNK_EID
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