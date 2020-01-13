use crate::imc::Message::*;
use crate::imc::{IMC_CONST_UNK_EID, DUNE_IMC_CONST_SYNC};

use bytes::BufMut;
use crate::imc::Header::Header;

const c_msg_id :u16 = 102;

pub enum ControlOperationEnum
{
    // Request Start of Logging.
    COP_REQUEST_START = 0,
    // Logging Started.
    COP_STARTED = 1,
    // Request Logging Stop.
    COP_REQUEST_STOP = 2,
    // Logging Stopped.
    COP_STOPPED = 3,
    // Request Current Log Name.
    COP_REQUEST_CURRENT_NAME = 4,
    // Current Log Name.
    COP_CURRENT_NAME = 5
}

impl ControlOperationEnum
{
    pub fn as_u8(&self) -> u8
    {
        match self {
            ControlOperationEnum::COP_REQUEST_START =>        0,
            ControlOperationEnum::COP_STARTED =>              1,
            ControlOperationEnum::COP_REQUEST_STOP =>         2,
            ControlOperationEnum::COP_STOPPED =>              3,
            ControlOperationEnum::COP_REQUEST_CURRENT_NAME => 4,
            ControlOperationEnum::COP_CURRENT_NAME =>         5,
        }
    }
}


pub(crate) struct LoggingControl
{
    pub header: Header,
    pub op: ControlOperationEnum,
    pub name: &'static str,
}

impl LoggingControl
{
    pub(crate) fn new() -> LoggingControl
    {
        let mut msg = LoggingControl {
            header : Header::new(c_msg_id),
            op: ControlOperationEnum::COP_REQUEST_START,
            name: ""
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LoggingControl
{
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16
    {
        c_msg_id
    }

    fn clear(&mut self)
    {
        self.name = "";
        self.op = ControlOperationEnum::COP_REQUEST_START;
    }

    fn fixed_serialization_size(&self) -> usize
    {
        1
    }

    fn dynamic_serialization_size(&self) -> usize
    {
        self.name.len() + 2
    }

    fn serialize(&self, bfr :&mut bytes::BytesMut)
    {
        self.header.serialize(bfr);
        bfr.put(self.op.as_u8());
        serialize_string!(bfr, self.name);
        serialize_footer(bfr);
    }
}