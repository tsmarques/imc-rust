use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::StoragePartition::StoragePartition;

use crate::packet::*;

/// Storage device information (e.g disk, usb flash, etc). NOTE: This is different from a partition
#[derive(Default)]
pub struct StorageDevice {
    /// IMC Header
    pub header: Header,

    /// Storage device's model (e.g. Samsung Flash Drive FIT)
    pub _device_model: String,

    /// Device size in MiB
    pub _size: u32,

    /// Device's path on the filesystem, e.g. /dev/sdb
    pub _path: String,

    /// Text description of the partition type, e.g. msdos, gpt, etc
    pub _ptype: String,

    /// List of partitions belonging to this device
    pub _partitions: MessageList<StoragePartition>,

    /// Flag to signal if this device is the main disk device
    pub _is_main_device: u8,
}

impl Message for StorageDevice {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = StorageDevice {
            header: Header::new(108),

            _device_model: Default::default(),
            _size: Default::default(),
            _path: Default::default(),
            _ptype: Default::default(),
            _partitions: vec![],
            _is_main_device: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = StorageDevice {
            header: hdr,

            _device_model: Default::default(),
            _size: Default::default(),
            _path: Default::default(),
            _ptype: Default::default(),
            _partitions: vec![],
            _is_main_device: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        108
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        108
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._device_model = Default::default();

        self._size = Default::default();

        self._path = Default::default();

        self._ptype = Default::default();

        self._partitions = Default::default();

        self._is_main_device = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._device_model.len() + 2;

        dyn_size += self._path.len() + 2;

        dyn_size += self._ptype.len() + 2;

        message_list_serialization_size!(dyn_size, self._partitions);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._device_model.as_bytes());
        bfr.put_u32_le(self._size);
        serialize_bytes!(bfr, self._path.as_bytes());
        serialize_bytes!(bfr, self._ptype.as_bytes());
        serialize_message_list!(bfr, self._partitions);
        bfr.put_u8(self._is_main_device);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._device_model);

        self._size = bfr.get_u32_le();

        deserialize_string!(bfr, self._path);

        deserialize_string!(bfr, self._ptype);

        for m in self._partitions.iter_mut() {
            m.deserialize_fields(bfr);
        }

        self._is_main_device = bfr.get_u8();
    }
}
