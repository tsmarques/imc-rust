//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::DesiredSpeed::DesiredSpeed;
use crate::DesiredZ::DesiredZ;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Flags.
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Use Location Reference.
    pub const FLAG_LOCATION: u32 = 0x01;
    /// Use Speed Reference.
    pub const FLAG_SPEED: u32 = 0x02;
    /// Use Z Reference.
    pub const FLAG_Z: u32 = 0x04;
    /// Use Radius Reference.
    pub const FLAG_RADIUS: u32 = 0x08;
    /// Use this Reference as Start Position for PathControler.
    pub const FLAG_START_POINT: u32 = 0x10;
    /// Use Current Position as Start Position for PathControler.
    pub const FLAG_DIRECT: u32 = 0x20;
    /// Flag Maneuver Completion.
    pub const FLAG_MANDONE: u32 = 0x80;
}

#[derive(Default)]
pub struct Reference {
    /// Message Header.
    pub _header: Header,
    /// Flags.
    pub _flags: u8,
    /// Speed Reference.
    pub _speed: Option<DesiredSpeed>,
    /// Z Reference.
    pub _z: Option<DesiredZ>,
    /// Latitude Reference.
    pub _lat: f64,
    /// Longitude Reference.
    pub _lon: f64,
    /// Radius.
    pub _radius: f32,
}

impl Message for Reference {
    fn new() -> Reference {
        let msg = Reference {
            _header: Header::new(479),
            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _radius: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        479
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        479
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
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
