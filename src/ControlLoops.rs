use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum EnableEnum {
    // Disable
    CL_DISABLE = 0,
    // Enable
    CL_ENABLE = 1,
}

impl EnableEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            CL_DISABLE => 0,
            CL_ENABLE => 1,
        }
    }
}

/// Enable or disable control loops.
#[derive(Default)]
pub struct ControlLoops {
    /// IMC Header
    pub header: Header,

    pub _enable: u8,

    /// Control loop mask.
    pub _mask: u32,

    /// Unsigned integer reference for the scope of the control loop message.
    /// Scope reference should only be set by a maneuver.
    /// Should be set to an always increasing reference at the time of dispatching this message.
    /// Lower level controllers must inherit the same scope reference sent by maneuver.
    /// This same scope reference must be sent down to lower control layers.
    pub _scope_ref: u32,
}

impl Message for ControlLoops {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = ControlLoops {
            header: hdr,

            _enable: Default::default(),
            _mask: Default::default(),
            _scope_ref: Default::default(),
        };

        msg.get_header()._mgid = 507;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = ControlLoops {
            header: Header::new(507),

            _enable: Default::default(),
            _mask: Default::default(),
            _scope_ref: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        507
    }

    fn id(&self) -> u16 {
        507
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._enable = Default::default();

        self._mask = Default::default();

        self._scope_ref = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._enable);
        bfr.put_u32_le(self._mask);
        bfr.put_u32_le(self._scope_ref);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
