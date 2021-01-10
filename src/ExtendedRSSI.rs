use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Measure of the RSSI by a networking device.
/// Indicates the gain or loss in the signal strenght due to the transmission
/// and reception equipment and the transmission medium and distance.
pub struct ExtendedRSSI {
    /// IMC Header
    pub header: Header,

    /// RSSI measurement.
    pub _value: f32,

    /// Indicates the units used for the RSSI value.
    pub _units: u8,
}

impl ExtendedRSSI {
    pub fn new() -> ExtendedRSSI {
        let mut msg = ExtendedRSSI {
            header: Header::new(183),

            _value: Default::default(),
            _units: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ExtendedRSSI {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        183
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
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f32_le(self._value);
        bfr.put_u8(self._units);

        serialize_footer(bfr);
    }
}
