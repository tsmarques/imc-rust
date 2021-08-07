use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// When the vehicle uses Long Base Line navigation, this message
/// notifies that a new range was received from one of the acoustics
/// transponders. The message fields are used to identify the range
/// value and the transponder name.
#[derive(Default)]
pub struct LblRange {
    /// IMC Header
    pub header: Header,

    /// Identification number of the acoustic transponder from which
    /// the range information was received.
    pub _id: u8,

    /// Distance to the acoustic transponder.
    pub _range: f32,
}

impl Message for LblRange {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = LblRange {
            header: hdr,

            _id: Default::default(),
            _range: Default::default(),
        };

        msg.get_header()._mgid = 200;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = LblRange {
            header: Header::new(200),

            _id: Default::default(),
            _range: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        200
    }

    fn id(&self) -> u16 {
        200
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._range = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_f32_le(self._range);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
