use bytes::{BytesMut, BufMut, LittleEndian};

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