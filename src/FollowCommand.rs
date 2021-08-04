use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for FollowCommand {}

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

impl FollowCommand {
    pub fn new() -> FollowCommand {
        let mut msg = FollowCommand {
            header: Header::new(496),

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _timeout: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for FollowCommand {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        496
    }

    fn clear(&mut self) {
        self.header.clear();

        self._control_src = Default::default();

        self._control_ent = Default::default();

        self._timeout = Default::default();
    }

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
}
