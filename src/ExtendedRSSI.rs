use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = ExtendedRSSI {
            header: Header::new(183),

            _value: Default::default(),
            _units: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = ExtendedRSSI {
            header: hdr,

            _value: Default::default(),
            _units: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        183
    }

    #[inline(always)]
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

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();

        self._units = bfr.get_u8();

        Ok(())
    }
}
