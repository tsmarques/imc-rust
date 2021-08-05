use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum EventTypeEnum {
    // Log Book Entry Added
    EVT_LOG_ENTRY = 1,
    // Plan Added
    EVT_PLAN_ADDED = 2,
    // Plan Removed
    EVT_PLAN_REMOVED = 3,
    // Plan Changed
    EVT_PLAN_CHANGED = 4,
    // Map feature added
    EVT_MAP_FEATURE_ADDED = 5,
    // Map feature removed
    EVT_MAP_FEATURE_REMOVED = 6,
    // Map feature changed
    EVT_MAP_FEATURE_CHANGED = 7,
    // The sender is now teleoperating the vehicle
    EVT_TELEOPERATION_STARTED = 8,
    // The sender stopped teleoperating the vehicle
    EVT_TELEOPERATION_ENDED = 9,
}

impl EventTypeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            EVT_LOG_ENTRY => 1,
            EVT_PLAN_ADDED => 2,
            EVT_PLAN_REMOVED => 3,
            EVT_PLAN_CHANGED => 4,
            EVT_MAP_FEATURE_ADDED => 5,
            EVT_MAP_FEATURE_REMOVED => 6,
            EVT_MAP_FEATURE_CHANGED => 7,
            EVT_TELEOPERATION_STARTED => 8,
            EVT_TELEOPERATION_ENDED => 9,
        }
    }
}

/// This message is used to signal events among running CCUs.
#[derive(Default)]
pub struct CcuEvent {
    /// IMC Header
    pub header: Header,

    pub _type: u8,

    pub _id: String,

    pub _arg: Option<Box<dyn Message>>,
}

impl CcuEvent {
    pub fn new() -> CcuEvent {
        let mut msg = CcuEvent {
            header: Header::new(606),

            _type: Default::default(),
            _id: Default::default(),
            _arg: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for CcuEvent {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        606
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._id = Default::default();

        match &mut self._arg {
            Some(field) => field.clear(),

            None => {}
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._id.len() + 2;

        match &self._arg {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        serialize_bytes!(bfr, self._id.as_bytes());
        match &self._arg {
            None => {}

            Some(m) => m.serialize_fields(bfr),
        };
    }
}
