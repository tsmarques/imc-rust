use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Promiscuous
    URF_PROMISCUOUS = 0x01,
    // Delayed
    URF_DELAYED = 0x02,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            URF_PROMISCUOUS => 0x01,
            URF_DELAYED => 0x02,
        }
    }
}

/// The data frame was transmitted to an acoustic modem other than
/// the one the acoustic modem driver is controlling.
#[derive(Default)]
pub struct UamRxFrame {
    /// IMC Header
    pub header: Header,

    /// The canonical name of the node that transmitted the data frame. If
    /// this name cannot be resolved the string 'unknown' shall be used.
    pub _sys_src: String,

    /// The canonical name of the destination node of the data frame. If
    /// this name cannot be resolved the string 'unknown' shall be used.
    pub _sys_dst: String,

    /// The data frame was transmitted using the DELAYED flag.
    pub _flags: u8,

    /// The actual received data frame.
    pub _data: Vec<u8>,
}

impl Message for UamRxFrame {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UamRxFrame {
            header: Header::new(815),

            _sys_src: Default::default(),
            _sys_dst: Default::default(),
            _flags: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UamRxFrame {
            header: hdr,

            _sys_src: Default::default(),
            _sys_dst: Default::default(),
            _flags: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        815
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        815
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._sys_src = Default::default();

        self._sys_dst = Default::default();

        self._flags = Default::default();

        self._data = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._sys_src.len() + 2;

        dyn_size += self._sys_dst.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._sys_src.as_bytes());
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._sys_src);

        deserialize_string!(bfr, self._sys_dst);

        self._flags = bfr.get_u8();

        deserialize_bytes!(bfr, self._data);
    }
}
