use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::LblBeacon::LblBeacon;

/// LBL Beacon position estimate.
#[derive(Default)]
pub struct LblEstimate {
    /// IMC Header
    pub header: Header,

    /// LBL Beacon configuration estimate.
    pub _beacon: Option<Box<LblBeacon>>,

    /// The North position offset of the NED field with respect to origin.
    pub _x: f32,

    /// The East position offset of the NED field with respect to origin.
    pub _y: f32,

    /// The North offset variance of the North/East/Down
    /// field with respect to LLH.
    pub _var_x: f32,

    /// The East offset variance of the North/East/Down
    /// field with respect to LLH.
    pub _var_y: f32,

    /// Distance between current LBL Beacon position and filter estimation.
    pub _distance: f32,
}

impl Message for LblEstimate {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = LblEstimate {
            header: Header::new(360),

            _beacon: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _var_x: Default::default(),
            _var_y: Default::default(),
            _distance: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = LblEstimate {
            header: hdr,

            _beacon: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _var_x: Default::default(),
            _var_y: Default::default(),
            _distance: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        360
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        360
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        match &mut self._beacon {
            Some(field) => field.clear(),

            None => {}
        }

        self._x = Default::default();

        self._y = Default::default();

        self._var_x = Default::default();

        self._var_y = Default::default();

        self._distance = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        20
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        match &self._beacon {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        match &self._beacon {
            None => {}

            Some(m) => m.serialize_fields(bfr),
        };
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._var_x);
        bfr.put_f32_le(self._var_y);
        bfr.put_f32_le(self._distance);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        match &mut self._beacon {
            None => {}

            Some(m) => m.deserialize_fields(bfr),
        };

        self._x = bfr.get_f32_le();

        self._y = bfr.get_f32_le();

        self._var_x = bfr.get_f32_le();

        self._var_y = bfr.get_f32_le();

        self._distance = bfr.get_f32_le();
    }
}
