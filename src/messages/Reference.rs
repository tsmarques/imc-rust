//###########################################################################
// Licensed under the Apache License, Version 2.0 (the "License");          #
// you may not use this file except in compliance with the License.         #
// You may obtain a copy of the License at                                  #
//                                                                          #
// http://www.apache.org/licenses/LICENSE-2.0                               #
//                                                                          #
// Unless required by applicable law or agreed to in writing, software      #
// distributed under the License is distributed on an "AS IS" BASIS,        #
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. #
// See the License for the specific language governing permissions and      #
// limitations under the License.                                           #
//###########################################################################
// Author: Tiago Sá Marques                                                 #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::packet::*;
use crate::DesiredSpeed;
use crate::DesiredZ;
use crate::Header;
use crate::Message;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Use Location Reference
    pub const FLAG_LOCATION: u32 = 0x01;
    /// Use Speed Reference
    pub const FLAG_SPEED: u32 = 0x02;
    /// Use Z Reference
    pub const FLAG_Z: u32 = 0x04;
    /// Use Radius Reference
    pub const FLAG_RADIUS: u32 = 0x08;
    /// Use this Reference as Start Position for PathControler
    pub const FLAG_START_POINT: u32 = 0x10;
    /// Use Current Position as Start Position for PathControler
    pub const FLAG_DIRECT: u32 = 0x20;
    /// Flag Maneuver Completion
    pub const FLAG_MANDONE: u32 = 0x80;
}

#[derive(Default, Clone)]
pub struct Reference {
    /// Message Header
    pub _header: Header,
    pub _flags: u8,
    pub _speed: Option<DesiredSpeed>,
    pub _z: Option<DesiredZ>,
    pub _lat: f64,
    pub _lon: f64,
    pub _radius: f32,
}

impl Message for Reference {
    fn new() -> Reference {
        Reference {
            _header: Header::new(479),
            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _radius: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        479
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        479
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&self) -> &Header {
        &self._header
    }

    fn get_mut_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_mut_header()._timestamp = ts;
        if let Some(m) = &mut self._speed {
            m.set_timestamp_secs(ts);
        }
        if let Some(m) = &mut self._z {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        if let Some(m) = &mut self._speed {
            m.set_source(src);
        }
        if let Some(m) = &mut self._z {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        if let Some(m) = &mut self._speed {
            m.set_source_ent(src_ent);
        }
        if let Some(m) = &mut self._z {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        if let Some(m) = &mut self._speed {
            m.set_destination(dst);
        }
        if let Some(m) = &mut self._z {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._speed {
            m.set_destination_ent(dst_ent);
        }
        if let Some(m) = &mut self._z {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(479);
        self._flags = Default::default();
        self._speed = Default::default();
        self._z = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._radius = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        21
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._speed);
        inline_message_serialization_size!(dyn_size, self._z);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._flags);
        serialize_inline_message!(bfr, self._speed);
        serialize_inline_message!(bfr, self._z);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._radius);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._flags = bfr.get_u8();
        self._speed = deserialize_inline_as::<DesiredSpeed>(bfr).ok();
        self._z = deserialize_inline_as::<DesiredZ>(bfr).ok();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._radius = bfr.get_f32_le();
        Ok(())
    }
}
