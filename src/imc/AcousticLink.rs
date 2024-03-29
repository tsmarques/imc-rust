use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

/// This message is used to report the perceived link quality to other
/// acoustic peers.
pub struct AcousticLink {
    /// IMC Header
    pub header: Header,

    /// The name of the peer on the other side of this link.
    pub _peer: String,

    /// RSSI is a signed floating point number. Higher RSSI values correspond to
    /// stronger signals.
    /// The signal strength is acceptable when measured RSSI values lie between
    /// -20 dB and -85 dB.
    pub _rssi: f32,

    /// Signal Integrity value illustrates distortion of the last received
    /// acoustic signal. It is calculated based on cross-correlation
    /// measurements.
    /// Higher *Signal Integrity Level* values correspond to less distorted
    /// signals. An acoustic link is considered weak if the *Signal Integrity
    /// Level* value is less than 100.
    pub _integrity: u16,
}

impl AcousticLink {
    pub fn new() -> AcousticLink {
        let mut msg = AcousticLink {
            header: Header::new(214),

            _peer: Default::default(),
            _rssi: Default::default(),
            _integrity: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for AcousticLink {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        214
    }

    fn clear(&mut self) {
        self.header.clear();

        self._peer = Default::default();

        self._rssi = Default::default();

        self._integrity = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._peer.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._peer.as_bytes());
        bfr.put_f32_le(self._rssi);
        bfr.put_u16_le(self._integrity);

        serialize_footer(bfr);
    }
}
