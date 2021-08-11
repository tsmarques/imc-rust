use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// This message is used to send a periodic update of values for
/// each remote action. If the action is not on the list the assumed
/// value is 0.
#[derive(Default)]
pub struct RemoteActions {
    /// IMC Header
    pub header: Header,

    /// List of values for each remote action (e.g: &quot;Propeller=0.6,PanTilt=0.75,Lights=1&quot;).
    pub _actions: String,
}

impl Message for RemoteActions {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = RemoteActions {
            header: Header::new(305),

            _actions: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = RemoteActions {
            header: hdr,

            _actions: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        305
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        305
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._actions = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._actions.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._actions.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._actions);
    }
}
