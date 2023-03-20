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
use crate::BeamConfig::BeamConfig;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Sidescan
    ST_SIDESCAN = 0,
    /// Echo Sounder
    ST_ECHOSOUNDER = 1,
    /// Multibeam
    ST_MULTIBEAM = 2,
}

/// This message contains the data acquired by a single sonar
/// measurement. The following describes the format used to
/// fill the data field used in this message. (Byte order is
/// little endian.)
/// **Sidescan:**
/// +------+-------------------+-----------+
/// | Data | Name              | Type      |
/// +======+===================+===========+
/// | A    | Ranges data       |   uintX_t |
/// +------+-------------------+-----------+
/// .. figure:: ../images/imc_sidescan.png
/// * The type *uintX_t* will depend on the number of bits per unit, and it should be a multiple of 8.
/// * Furthermore, for now, 32 bits is the highest value of bits per unit supported.
/// **Multibeam:**
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | Index| Section| Name                    | Type    | Comments                                                             |
/// +======+========+=========================+=========+======================================================================+
/// | 1    | H1     | Number of points        | uint16_t| Number of data points                                                |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | 2    | H2     | Start angle             | fp32_t  | In radians                                                           |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | 3    | H3     | Flags                   | uint8_t | Refer to next table                                                  |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | 4    | H4 ?   | Angle scale factor      | fp32_t  | Used for angle steps in radians                                      |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | 5    | H5 ?   | Intensities scale factor| fp32_t  |                                                                      |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | 6    | D1 ?   | Angle steps[H1]         | uint16_t| Values in radians                                                    |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | 7    | D2     | Ranges[H1]              | uintX_t | Ranges data points (scale factor from common field "Scaling Factor") |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// | 8    | D3 ?   | Intensities[H1]         | uintX_t | Intensities data points                                              |
/// +------+--------+-------------------------+---------+----------------------------------------------------------------------+
/// +--------+------------------+-----+
/// | Section| Flag Label       | Bit |
/// +========+==================+=====+
/// | H3.1   | Intensities flag | 0   |
/// +--------+------------------+-----+
/// | H3.2   | Angle step flag  | 1   |
/// +--------+------------------+-----+
/// .. figure:: ../images/imc_multibeam.png
/// *Notes:*
/// * Each angle at step *i* can be calculated is defined by:
/// .. code-block:: python
/// angle[i] = H2_start_angle + (32-bit sum of D1_angle_step[0] through D1_angle_step[i]) * H4_scaling_factor
/// * If bit H3.1 is not set then sections H5 and D3 won't exist.
/// * If bit H3.2 is not set then sections H4 and D1 won't exist. In case this bit is set, then the angle steps is read from field "Beam Width" from "Beam Configuration".
/// * The type *uintX_t* will depend on the number of bits per unit, and it should be a multiple of 8.
/// * Furthermore, for now, 32 bits is the highest value of bits per unit supported.
/// *How to write ranges and intensities data:*
/// .. code-block:: python
/// :linenos:
/// data_unit = (Integer) (data_value / scale_factor);
/// bytes_per_unit = bits_per_unit / 8;
/// LOOP: i = 0, until i = bytes_per_unit
/// byte[i] = (data_unit >> 8 * i) & 0xFF);
/// write(byte);
/// **Common:**
#[derive(Default, Clone)]
pub struct SonarData {
    /// Message Header
    pub _header: Header,
    /// Type of sonar.
    pub _type: u8,
    /// Operating frequency.
    pub _frequency: u32,
    /// Minimum range.
    pub _min_range: u16,
    /// Maximum range.
    pub _max_range: u16,
    /// Size of the data unit. (Should be multiple of 8)
    pub _bits_per_point: u8,
    /// Scaling factor used to multiply each data unit to restore the
    /// original floating point value.
    pub _scale_factor: f32,
    /// Beam configuration of the device.
    pub _beam_config: MessageList<BeamConfig>,
    /// Data acquired by the measurement.
    pub _data: Vec<u8>,
}

impl Message for SonarData {
    fn new() -> SonarData {
        SonarData {
            _header: Header::new(276),
            _type: Default::default(),
            _frequency: Default::default(),
            _min_range: Default::default(),
            _max_range: Default::default(),
            _bits_per_point: Default::default(),
            _scale_factor: Default::default(),
            _beam_config: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        276
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        276
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
        for m in &mut self._beam_config {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        for m in &mut self._beam_config {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        for m in &mut self._beam_config {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        for m in &mut self._beam_config {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        for m in &mut self._beam_config {
            m.set_destination_ent(dst_ent);
        }
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
