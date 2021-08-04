use bytes::BufMut;

#[derive(Default)]
pub struct Header {
    /// The synchronization number marks the beginning of a packet.
    ///  
    /// It denotes the packet API version and can be used to deduce
    /// the byte order of the sending host.
    ///  
    /// It encodes value 0xFE[major][minor] where [major] equals the
    /// major version number of the protocol and [minor] equals the
    /// minor version of the protocol.
    ///  
    /// The packet recipient is responsible for the correct
    /// interpretation of the synchronization number and byte order
    /// conversions.
    pub _sync: u16,

    /// The identification number of the message contained in the
    /// packet. This field is used for correct message interpretation
    /// and deserialization.
    pub _mgid: u16,

    /// The size of the message data in the packet.
    pub _size: u16,

    /// The time when the packet was sent, as seen by the packet
    /// dispatcher. The number of seconds is represented in Universal
    /// Coordinated Time (UCT) in seconds since Jan 1, 1970 using IEEE
    /// double precision floating point numbers.
    pub _timestamp: f64,

    /// The Source IMC system ID.
    pub _src: u16,

    /// The entity generating this message at the source address.
    pub _src_ent: u8,

    /// The Destination IMC system ID.
    pub _dst: u16,

    /// The entity that should process this message at the destination
    /// address.
    pub _dst_ent: u8,
}

impl Header {
    pub(crate) fn new(msg_id: u16) -> Header {
        let mut header = Header {
            _sync: 0xFE54 as u16,
            _mgid: Default::default(),
            _size: Default::default(),
            _timestamp: Default::default(),
            _src: Default::default(),
            _src_ent: Default::default(),
            _dst: Default::default(),
            _dst_ent: Default::default(),
        };

        header
    }

    pub fn clear(&mut self) {
        self._sync = Default::default();

        self._mgid = Default::default();

        self._size = Default::default();

        self._timestamp = Default::default();

        self._src = Default::default();

        self._src_ent = Default::default();

        self._dst = Default::default();

        self._dst_ent = Default::default();
    }

    pub fn serialize(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._sync);
        bfr.put_u16_le(self._mgid);
        bfr.put_u16_le(self._size);
        bfr.put_f64_le(self._timestamp);
        bfr.put_u16_le(self._src);
        bfr.put_u8(self._src_ent);
        bfr.put_u16_le(self._dst);
        bfr.put_u8(self._dst_ent);
    }
}
