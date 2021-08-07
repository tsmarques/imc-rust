use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Measure of the RSSI by a networking device.
/// Indicates the gain or loss in the signal strenght due to the transmission
/// and reception equipment and the transmission medium and distance.
#[derive(Default)]
pub struct ExtendedRSSI {
    /// IMC Header
    pub header: Header,

    /// RSSI measurement.
    pub _value: f32,

    /// Indicates the units used for the RSSI value.
    pub _units: u8,
}

impl Message for ExtendedRSSI {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = ExtendedRSSI {
            header: hdr,

            _value: Default::default(),
            _units: Default::default(),
        };

        msg.get_header()._mgid = 183;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = ExtendedRSSI {
            header: Header::new(183),

            _value: Default::default(),
            _units: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        183
    }

    fn id(&self) -> u16 {
        183
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();

        self._units = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_u8(self._units);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
