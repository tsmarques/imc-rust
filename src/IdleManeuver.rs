use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

use crate::packet::ImcError;
use crate::packet::*;

/// message-group: Maneuver
// impl Maneuver for IdleManeuver { }

/// Causes the vehicle to stay idle for some time.
/// message-group: Maneuver
#[derive(Default)]
pub struct IdleManeuver {
    /// IMC Header
    pub header: Header,

    /// Optional duration of the Idle maneuver. Use '0' for unlimited
    /// duration time (maneuver will have to be explicitly stopped).
    pub _duration: u16,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for IdleManeuver {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = IdleManeuver {
            header: Header::new(454),

            _duration: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = IdleManeuver {
            header: hdr,

            _duration: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        454
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        454
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._duration = Default::default();

        self._custom = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._duration);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._duration = bfr.get_u16_le();

        deserialize_string!(bfr, self._custom);

        Ok(())
    }
}
