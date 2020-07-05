use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};
use bytes::{BufMut, BytesMut, LittleEndian};

// The packet header contains handling information in the form of
// supplemental fields, it is always placed at the beginning of a
// packet.

pub struct Header {
    // The synchronization number marks the beginning of a packet.
    //
    // It denotes the packet API version and can be used to deduce
    // the byte order of the sending host.
    //
    // It encodes value 0xFE[major][minor] where [major] equals the
    // major version number of the protocol and [minor] equals the
    // minor version of the protocol.
    //
    // The packet recipient is responsible for the correct
    // interpretation of the synchronization number and byte order
    // conversions.
    pub sync: u16,

    // The identification number of the message contained in the
    // packet. This field is used for correct message interpretation
    // and deserialization.
    pub mgid: u16,

    // The size of the message data in the packet.
    pub size: u16,

    // The time when the packet was sent, as seen by the packet
    // dispatcher. The number of seconds is represented in Universal
    // Coordinated Time (UCT) in seconds since Jan 1, 1970 using IEEE
    // double precision floating point numbers.
    pub timestamp: f64,

    // The Source IMC system ID.
    pub src: u16,

    // The entity generating this message at the source address.
    pub src_ent: u8,

    // The Destination IMC system ID.
    pub dst: u16,

    // The entity that should process this message at the destination
    // address.
    pub dst_ent: u8,
}

impl Header {
    pub(crate) fn new(msg_id: u16) -> Header {
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

    pub fn serialize(&self, bfr: &mut bytes::BytesMut) {
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
