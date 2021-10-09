use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// Definition of operational limits.
#[derive(Default)]
pub struct OperationalLimits {
    /// IMC Header
    pub header: Header,

    pub _mask: u8,

    pub _max_depth: f32,

    pub _min_altitude: f32,

    pub _max_altitude: f32,

    pub _min_speed: f32,

    pub _max_speed: f32,

    pub _max_vrate: f32,

    pub _lat: f64,

    pub _lon: f64,

    pub _orientation: f32,

    pub _width: f32,

    pub _length: f32,
}

impl Message for OperationalLimits {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = OperationalLimits {
            header: Header::new(504),

            _mask: Default::default(),
            _max_depth: Default::default(),
            _min_altitude: Default::default(),
            _max_altitude: Default::default(),
            _min_speed: Default::default(),
            _max_speed: Default::default(),
            _max_vrate: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _orientation: Default::default(),
            _width: Default::default(),
            _length: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = OperationalLimits {
            header: hdr,

            _mask: Default::default(),
            _max_depth: Default::default(),
            _min_altitude: Default::default(),
            _max_altitude: Default::default(),
            _min_speed: Default::default(),
            _max_speed: Default::default(),
            _max_vrate: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _orientation: Default::default(),
            _width: Default::default(),
            _length: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        504
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        504
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._mask = Default::default();

        self._max_depth = Default::default();

        self._min_altitude = Default::default();

        self._max_altitude = Default::default();

        self._min_speed = Default::default();

        self._max_speed = Default::default();

        self._max_vrate = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._orientation = Default::default();

        self._width = Default::default();

        self._length = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        53
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._mask);
        bfr.put_f32_le(self._max_depth);
        bfr.put_f32_le(self._min_altitude);
        bfr.put_f32_le(self._max_altitude);
        bfr.put_f32_le(self._min_speed);
        bfr.put_f32_le(self._max_speed);
        bfr.put_f32_le(self._max_vrate);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._orientation);
        bfr.put_f32_le(self._width);
        bfr.put_f32_le(self._length);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._mask = bfr.get_u8();

        self._max_depth = bfr.get_f32_le();

        self._min_altitude = bfr.get_f32_le();

        self._max_altitude = bfr.get_f32_le();

        self._min_speed = bfr.get_f32_le();

        self._max_speed = bfr.get_f32_le();

        self._max_vrate = bfr.get_f32_le();

        self._lat = bfr.get_f64_le();

        self._lon = bfr.get_f64_le();

        self._orientation = bfr.get_f32_le();

        self._width = bfr.get_f32_le();

        self._length = bfr.get_f32_le();
    }
}
