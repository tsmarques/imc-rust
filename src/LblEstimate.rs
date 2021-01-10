use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::LblBeacon::LblBeacon;

/// LBL Beacon position estimate.
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

impl LblEstimate {
    pub fn new() -> LblEstimate {
        let mut msg = LblEstimate {
            header: Header::new(360),

            _beacon: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _var_x: Default::default(),
            _var_y: Default::default(),
            _distance: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LblEstimate {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        360
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

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        match &self._beacon {
            Some(field) => field.serialize(bfr),

            None => {}
        };
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._var_x);
        bfr.put_f32_le(self._var_y);
        bfr.put_f32_le(self._distance);

        serialize_footer(bfr);
    }
}
