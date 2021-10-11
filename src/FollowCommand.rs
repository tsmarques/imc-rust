use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

use crate::packet::ImcError;
use crate::packet::*;

/// message-group: Maneuver
// impl Maneuver for FollowCommand { }

/// This maneuver follows a direct command given by an external entity.
/// message-group: Maneuver
#[derive(Default)]
pub struct FollowCommand {
    /// IMC Header
    pub header: Header,

    /// The IMC identifier of the source system that is allowed to provide command to this maneuver.
    /// If the value ''0xFFFF'' is used, any system is allowed to command references.
    pub _control_src: u16,

    /// The entity identifier of the entity that is allowed to provide commands to this maneuver.
    /// If the value ''0xFF'' is used, any entity is allowed to command references.
    pub _control_ent: u8,

    /// The ammount of time, in seconds, after which the maneuver will be terminated if no new command has
    /// been received. In other words, the controlling entity should send command updates in shorter periods than
    /// 'timeout'.
    pub _timeout: f32,
}

impl Message for FollowCommand {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = FollowCommand {
            header: Header::new(496),

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _timeout: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = FollowCommand {
            header: hdr,

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _timeout: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        496
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        496
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._control_src = Default::default();

        self._control_ent = Default::default();

        self._timeout = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        7
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._control_src);
        bfr.put_u8(self._control_ent);
        bfr.put_f32_le(self._timeout);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._control_src = bfr.get_u16_le();

        self._control_ent = bfr.get_u8();

        self._timeout = bfr.get_f32_le();

        Ok(())
    }
}
