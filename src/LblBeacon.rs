use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Position and configuration of an LBL transponder (beacon).
#[derive(Default)]
pub struct LblBeacon {
    /// IMC Header
    pub header: Header,

    /// Name/Label of the acoustic transponder.
    pub _beacon: String,

    /// WGS-84 Latitude coordinate.
    pub _lat: f64,

    /// WGS-84 Longitude coordinate.
    pub _lon: f64,

    /// The beacon's depth.
    pub _depth: f32,

    /// Interrogation channel.
    pub _query_channel: u8,

    /// Reply channel.
    pub _reply_channel: u8,

    /// Transponder delay.
    pub _transponder_delay: u8,
}

impl Message for LblBeacon {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = LblBeacon {
            header: Header::new(202),

            _beacon: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _depth: Default::default(),
            _query_channel: Default::default(),
            _reply_channel: Default::default(),
            _transponder_delay: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = LblBeacon {
            header: hdr,

            _beacon: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _depth: Default::default(),
            _query_channel: Default::default(),
            _reply_channel: Default::default(),
            _transponder_delay: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        202
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        202
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._beacon = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._depth = Default::default();
        self._query_channel = Default::default();
        self._reply_channel = Default::default();
        self._transponder_delay = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        23
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._beacon.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._beacon.as_bytes());
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._depth);
        bfr.put_u8(self._query_channel);
        bfr.put_u8(self._reply_channel);
        bfr.put_u8(self._transponder_delay);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._beacon);
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._depth = bfr.get_f32_le();
        self._query_channel = bfr.get_u8();
        self._reply_channel = bfr.get_u8();
        self._transponder_delay = bfr.get_u8();

        Ok(())
    }
}
