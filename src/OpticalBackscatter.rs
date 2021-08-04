#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// The optical backscattering coefficient refers to all the photons that have been redirected in the backward directions
/// when a photon of light propagates in water and interacts with a &quot;particle&quot; (varying from water molecules to fish).
#[derive(Default)]
pub struct OpticalBackscatter {
    /// IMC Header
    pub header: Header,

    /// Optical Backscattering Coefficient.
    pub _value: f32,
}

impl OpticalBackscatter {
    pub fn new() -> OpticalBackscatter {
        let mut msg = OpticalBackscatter {
            header: Header::new(904),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for OpticalBackscatter {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        904
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }
}
