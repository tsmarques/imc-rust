#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for FollowReference {}

/// This maneuver follows a reference given by an external entity.
/// message-group: Maneuver
#[derive(Default)]
pub struct FollowReference {
    /// IMC Header
    pub header: Header,

    /// The IMC identifier of the source system that is allowed to provide references to this maneuver.
    /// If the value ''0xFFFF'' is used, any system is allowed to command references.
    pub _control_src: u16,

    /// The entity identifier of the entity that is allowed to provide references to this maneuver.
    /// If the value ''0xFF'' is used, any entity is allowed to command references.
    pub _control_ent: u8,

    /// The ammount of time, in seconds, after which the maneuver will be terminated if no reference has
    /// been received. In other words, the controlling entity should send reference updates in shorter periods than
    /// 'timeout'.
    pub _timeout: f32,

    /// Whenever an intended reference is achieved, this maneuver will maintain the vehicle in vaticiny of that
    /// location. The loiter radius is used to define the radius of this (xy) area.
    pub _loiter_radius: f32,

    /// Similarly to Loiter Radius, this field is used to define the &quot;z&quot; distance considered to be inside the vacitiny of
    /// the target location. An AUV may, for instance, be floating until it more than z units above the current reference,
    /// in which case it actively changes its position in order to achieve the desired depth / altitude.
    pub _altitude_interval: f32,
}

impl FollowReference {
    pub fn new() -> FollowReference {
        let mut msg = FollowReference {
            header: Header::new(478),

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _timeout: Default::default(),
            _loiter_radius: Default::default(),
            _altitude_interval: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for FollowReference {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        478
    }

    fn clear(&mut self) {
        self.header.clear();

        self._control_src = Default::default();

        self._control_ent = Default::default();

        self._timeout = Default::default();

        self._loiter_radius = Default::default();

        self._altitude_interval = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        15
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._control_src);
        bfr.put_u8(self._control_ent);
        bfr.put_f32_le(self._timeout);
        bfr.put_f32_le(self._loiter_radius);
        bfr.put_f32_le(self._altitude_interval);
    }
}
