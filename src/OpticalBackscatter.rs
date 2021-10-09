use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

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

impl Message for OpticalBackscatter {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = OpticalBackscatter {
            header: Header::new(904),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = OpticalBackscatter {
            header: hdr,

            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        904
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        904
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_f32_le();
    }
}
