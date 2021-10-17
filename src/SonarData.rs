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
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::BeamConfig::BeamConfig;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type.
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Sidescan.
    ST_SIDESCAN = 0,
    /// Echo Sounder.
    ST_ECHOSOUNDER = 1,
    /// Multibeam.
    ST_MULTIBEAM = 2,
}

/// This message contains the data acquired by a single sonar
/// measurement.
#[derive(Default)]
pub struct SonarData {
    /// Message Header.
    pub _header: Header,
    /// Type.
    pub _type: u8,
    /// Frequency.
    pub _frequency: u32,
    /// Minimum Range.
    pub _min_range: u16,
    /// Maximum Range.
    pub _max_range: u16,
    /// Bits Per Data Point.
    pub _bits_per_point: u8,
    /// Scaling Factor.
    pub _scale_factor: f32,
    /// Beam Configuration.
    pub _beam_config: MessageList<BeamConfig>,
    /// Data.
    pub _data: Vec<u8>,
}

impl Message for SonarData {
    fn new() -> SonarData {
        let msg = SonarData {
            _header: Header::new(276),
            _type: Default::default(),
            _frequency: Default::default(),
            _min_range: Default::default(),
            _max_range: Default::default(),
            _bits_per_point: Default::default(),
            _scale_factor: Default::default(),
            _beam_config: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        276
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        276
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(276);
        self._type = Default::default();
        self._frequency = Default::default();
        self._min_range = Default::default();
        self._max_range = Default::default();
        self._bits_per_point = Default::default();
        self._scale_factor = Default::default();
        self._beam_config = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        14
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._beam_config);
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u32_le(self._frequency);
        bfr.put_u16_le(self._min_range);
        bfr.put_u16_le(self._max_range);
        bfr.put_u8(self._bits_per_point);
        bfr.put_f32_le(self._scale_factor);
        serialize_message_list!(bfr, self._beam_config);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._frequency = bfr.get_u32_le();
        self._min_range = bfr.get_u16_le();
        self._max_range = bfr.get_u16_le();
        self._bits_per_point = bfr.get_u8();
        self._scale_factor = bfr.get_f32_le();
        self._beam_config = deserialize_message_list_as::<BeamConfig>(bfr)?;
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
