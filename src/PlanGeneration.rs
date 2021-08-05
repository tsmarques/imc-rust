use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum CommandEnum {
    // Generate
    CMD_GENERATE = 0,
    // Execute
    CMD_EXECUTE = 1,
}

impl CommandEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            CMD_GENERATE => 0,
            CMD_EXECUTE => 1,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // Request
    OP_REQUEST = 0,
    // Error
    OP_ERROR = 1,
    // Success
    OP_SUCCESS = 2,
}

impl OperationEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            OP_REQUEST => 0,
            OP_ERROR => 1,
            OP_SUCCESS => 2,
        }
    }
}

/// Some error has occurred while executing the command. The
/// error can be found in the 'params' tuplelist (under the
/// key 'error').
#[derive(Default)]
pub struct PlanGeneration {
    /// IMC Header
    pub header: Header,

    /// Generate the plan and store it in the PlanDB.
    pub _cmd: u8,

    /// The requested command was executed successfully.
    pub _op: u8,

    /// The name of the plan to be generated.
    pub _plan_id: String,

    /// An optional list of parameters to be used by the plan
    /// generation module.
    pub _params: String,
}

impl PlanGeneration {
    pub fn new() -> PlanGeneration {
        let mut msg = PlanGeneration {
            header: Header::new(562),

            _cmd: Default::default(),
            _op: Default::default(),
            _plan_id: Default::default(),
            _params: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanGeneration {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        562
    }

    fn clear(&mut self) {
        self.header.clear();

        self._cmd = Default::default();

        self._op = Default::default();

        self._plan_id = Default::default();

        self._params = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._plan_id.len() + 2;

        dyn_size += self._params.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._cmd);
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        serialize_bytes!(bfr, self._params.as_bytes());
    }
}
