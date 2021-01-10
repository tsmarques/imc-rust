use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// This message is used to store the various polygon vertices for
/// CoverArea maneuvers.
pub struct PolygonVertex {
    /// IMC Header
    pub header: Header,

    /// WGS-84 Latitude for start point.
    pub _lat: f64,

    /// WGS-84 Longitude for start point.
    pub _lon: f64,
}

impl PolygonVertex {
    pub fn new() -> PolygonVertex {
        let mut msg = PolygonVertex {
            header: Header::new(474),

            _lat: Default::default(),
            _lon: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PolygonVertex {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        474
    }

    fn clear(&mut self) {
        self.header.clear();

        self._lat = Default::default();

        self._lon = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);

        serialize_footer(bfr);
    }
}
