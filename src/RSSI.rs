use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Measure of the RSSI by a networking device.
/// Indicates the gain or loss in the signal strength due to the transmission and reception equipment and the transmission medium and distance.
#[derive(Default)]
pub struct RSSI {
    /// IMC Header
    pub header: Header,

    /// RSSI measurement.
    pub _value: f32,
}

impl Message for RSSI {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = RSSI {
            header: hdr,

            _value: Default::default(),
        };

        msg.get_header()._mgid = 153;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = RSSI {
            header: Header::new(153),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        153
    }

    fn id(&self) -> u16 {
        153
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
