use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum StatusEnum {
    // Mounted
    SDST_MOUNTED = 0,
    // Unmounted
    SDST_UNMOUNTED = 1,
    // Unknown
    SDST_UNKWN = 2,
}

impl StatusEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            SDST_MOUNTED => 0,
            SDST_UNMOUNTED => 1,
            SDST_UNKWN => 2,
        }
    }
}

/// Storage device is unmounted
#[derive(Default)]
pub struct StoragePartition {
    /// IMC Header
    pub header: Header,

    /// Partition path, e.g. /dev/sdb1 or nvme0n1p1.
    pub _part_path: String,

    /// User defined label
    pub _label: String,

    /// Partition size in MiB.
    pub _size: u32,

    /// Used partition size, as a percentage.
    pub _used_size: f32,

    /// Text description of the filesystem type, e.g. ext4. It should match
    /// the real name of the type.
    pub _fstype: String,

    /// Storage device state is unknown
    pub _status: u8,
}

impl Message for StoragePartition {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = StoragePartition {
            header: Header::new(109),

            _part_path: Default::default(),
            _label: Default::default(),
            _size: Default::default(),
            _used_size: Default::default(),
            _fstype: Default::default(),
            _status: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = StoragePartition {
            header: hdr,

            _part_path: Default::default(),
            _label: Default::default(),
            _size: Default::default(),
            _used_size: Default::default(),
            _fstype: Default::default(),
            _status: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        109
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        109
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._part_path = Default::default();

        self._label = Default::default();

        self._size = Default::default();

        self._used_size = Default::default();

        self._fstype = Default::default();

        self._status = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._part_path.len() + 2;

        dyn_size += self._label.len() + 2;

        dyn_size += self._fstype.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._part_path.as_bytes());
        serialize_bytes!(bfr, self._label.as_bytes());
        bfr.put_u32_le(self._size);
        bfr.put_f32_le(self._used_size);
        serialize_bytes!(bfr, self._fstype.as_bytes());
        bfr.put_u8(self._status);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._part_path);

        deserialize_string!(bfr, self._label);

        self._size = bfr.get_u32_le();

        self._used_size = bfr.get_f32_le();

        deserialize_string!(bfr, self._fstype);

        self._status = bfr.get_u8();

        Ok(())
    }
}
