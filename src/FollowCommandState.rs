use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::Command::Command;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Waiting for first command
    FC_WAIT = 1,
    // Moving towards received command
    FC_MOVING = 2,
    // Speed command is zero
    FC_STOPPED = 3,
    // Command is out of safe bounds
    FC_BAD_COMMAND = 4,
    // Controlling system timed out
    FC_TIMEOUT = 5,
}

/// Maneuver will be terminated since timeout was exceeded.
#[derive(Default)]
pub struct FollowCommandState {
    /// IMC Header
    pub header: Header,

    /// The IMC identifier of the source system that is allowed to control the vehicle.
    /// If the value ''0xFFFF'' is used, any system is allowed to command references.
    pub _control_src: u16,

    /// The entity identifier of the entity that is allowed to control the vehicle.
    /// If the value ''0xFF'' is used, any entity is allowed to command references.
    pub _control_ent: u8,

    /// Command currently being followed.
    pub _command: Option<Command>,

    /// Command provided breaks system's physical limitations.
    pub _state: u8,
}

impl Message for FollowCommandState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = FollowCommandState {
            header: Header::new(498),

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _command: Default::default(),
            _state: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = FollowCommandState {
            header: hdr,

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _command: Default::default(),
            _state: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        498
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        498
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._control_src = Default::default();
        self._control_ent = Default::default();
        self._command = Default::default();
        self._state = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        inline_message_serialization_size!(dyn_size, self._command);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._control_src);
        bfr.put_u8(self._control_ent);
        serialize_inline_message!(bfr, self._command);
        bfr.put_u8(self._state);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._control_src = bfr.get_u16_le();
        self._control_ent = bfr.get_u8();
        self._command = deserialize_inline_as::<Command>(bfr).ok();
        self._state = bfr.get_u8();

        Ok(())
    }
}
