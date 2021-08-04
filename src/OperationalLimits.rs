#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

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

impl OperationalLimits {
    pub fn new() -> OperationalLimits {
        let mut msg = OperationalLimits {
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

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for OperationalLimits {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        504
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

    fn fixed_serialization_size(&self) -> usize {
        53
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
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
}
