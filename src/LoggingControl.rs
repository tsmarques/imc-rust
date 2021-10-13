use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum ControlOperationEnum {
    // Request Start of Logging
    COP_REQUEST_START = 0,
    // Logging Started
    COP_STARTED = 1,
    // Request Logging Stop
    COP_REQUEST_STOP = 2,
    // Logging Stopped
    COP_STOPPED = 3,
    // Request Current Log Name
    COP_REQUEST_CURRENT_NAME = 4,
    // Current Log Name
    COP_CURRENT_NAME = 5,
}

#[allow(non_camel_case_types)]
pub mod ControlOperation {
    // Request Start of Logging
    pub const COP_REQUEST_START: u32 = 0;
    // Logging Started
    pub const COP_STARTED: u32 = 1;
    // Request Logging Stop
    pub const COP_REQUEST_STOP: u32 = 2;
    // Logging Stopped
    pub const COP_STOPPED: u32 = 3;
    // Request Current Log Name
    pub const COP_REQUEST_CURRENT_NAME: u32 = 4;
    // Current Log Name
    pub const COP_CURRENT_NAME: u32 = 5;
}

/// This operation instructs the logging manager to send a
/// message with operation CURRENT_NAME containing the complete
/// name of the current log in the field 'name'. The field
/// 'name' with this operation type has no meaning.
#[derive(Default)]
pub struct LoggingControl {
    /// IMC Header
    pub header: Header,

    /// The logging manager will send a message with this operation
    /// when asked via the REQUEST_CURRENT_NAME operation. The field
    /// 'name' contains the complete name of the log.
    pub _op: u8,

    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _name: String,
}

impl Message for LoggingControl {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = LoggingControl {
            header: Header::new(102),

            _op: Default::default(),
            _name: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = LoggingControl {
            header: hdr,

            _op: Default::default(),
            _name: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        102
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        102
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();
        self._name = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._name.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._name);

        Ok(())
    }
}
