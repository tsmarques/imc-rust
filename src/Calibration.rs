use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Initiate overall calibration of a vehicle.
#[derive(Default)]
pub struct Calibration {
    /// IMC Header
    pub header: Header,

    /// Duration of calibration.
    pub _duration: u16,
}

impl Message for Calibration {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Calibration {
            header: hdr,

            _duration: Default::default(),
        };

        msg.get_header()._mgid = 506;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = Calibration {
            header: Header::new(506),

            _duration: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        506
    }

    fn id(&self) -> u16 {
        506
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._duration = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._duration);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
