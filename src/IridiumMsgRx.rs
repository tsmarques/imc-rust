use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[derive(Default)]
pub struct IridiumMsgRx {
    /// IMC Header
    pub header: Header,

    /// The unique identifier of this message's origin device (e.g. lauv-xtreme-2, manta-0).
    pub _origin: String,

    /// Timestamp (Epoch time).
    pub _htime: f64,

    pub _lat: f64,

    pub _lon: f64,

    /// Message data.
    pub _data: Vec<u8>,
}

impl Message for IridiumMsgRx {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = IridiumMsgRx {
            header: Header::new(170),

            _origin: Default::default(),
            _htime: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = IridiumMsgRx {
            header: hdr,

            _origin: Default::default(),
            _htime: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        170
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        170
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._origin = Default::default();

        self._htime = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._data = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        24
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._origin.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._origin.as_bytes());
        bfr.put_f64_le(self._htime);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._origin);

        self._htime = bfr.get_f64_le();

        self._lat = bfr.get_f64_le();

        self._lon = bfr.get_f64_le();

        deserialize_bytes!(bfr, self._data);

        Ok(())
    }
}
