use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum TypeEnum {
    // Request
    SCTR_REQUEST = 0,
    // Reply -- Success
    SCTR_SUCCESS = 1,
    // Reply -- Failure
    SCTR_FAILURE = 2,
    // Reply -- In Progress
    SCTR_IN_PROGRESS = 3,
}

impl TypeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            SCTR_REQUEST => 0,
            SCTR_SUCCESS => 1,
            SCTR_FAILURE => 2,
            SCTR_IN_PROGRESS => 3,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // List
    SOP_LIST = 0,
    // Mount
    SOP_MOUNT = 1,
    // Unmount
    SOP_UMOUNT = 2,
    // Format
    SOP_FORMAT = 3,
}

impl OperationEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            SOP_LIST => 0,
            SOP_MOUNT => 1,
            SOP_UMOUNT => 2,
            SOP_FORMAT => 3,
        }
    }
}

/// Unmount the selected storage device
#[derive(Default)]
pub struct StorageControl {
    /// IMC Header
    pub header: Header,

    /// Indicates if the message is a request, or a reply to a
    /// previous request.
    pub _type: u8,

    /// Format the selected storage device
    pub _op: u8,

    /// Request ID. This may be used by interfacing modules,
    /// e.g. using sequence counters, to annotate requests and
    /// appropriately identify replies
    pub _request_id: u16,

    /// Identifier for the storage device to operate on
    pub _device_id: String,

    /// Human-readable complementary information. For example this
    /// may be used to detail reasons for failure, or to report
    /// in-progress information.
    pub _info: String,
}

impl Message for StorageControl {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = StorageControl {
            header: Header::new(107),

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _device_id: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = StorageControl {
            header: hdr,

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _device_id: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        107
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        107
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._op = Default::default();

        self._request_id = Default::default();

        self._device_id = Default::default();

        self._info = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._device_id.len() + 2;

        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._op);
        bfr.put_u16_le(self._request_id);
        serialize_bytes!(bfr, self._device_id.as_bytes());
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._type = bfr.get_u8();

        self._op = bfr.get_u8();

        self._request_id = bfr.get_u16_le();

        deserialize_string!(bfr, self._device_id);

        deserialize_string!(bfr, self._info);
    }
}
