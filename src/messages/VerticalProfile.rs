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
use crate::Header;
use crate::Message;
use crate::MessageList;
use crate::ProfileSample;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Parameter
#[allow(non_camel_case_types)]
pub enum ParameterEnum {
    /// Temperature
    PROF_TEMPERATURE = 0,
    /// Salinity
    PROF_SALINITY = 1,
    /// Conductivity
    PROF_CONDUCTIVITY = 2,
    /// pH
    PROF_PH = 3,
    /// Redox
    PROF_REDOX = 4,
    /// Chlorophyll
    PROF_CHLOROPHYLL = 5,
    /// Turbidity
    PROF_TURBIDITY = 6,
}

/// This message is used to store historic profiles for water parameters: Temperature, Salinity, Chlorophyll...
#[derive(Default, Clone)]
pub struct VerticalProfile {
    /// Message Header
    pub _header: Header,
    /// Water parameter used to calculate the vertical profile.
    pub _parameter: u8,
    pub _numSamples: u8,
    pub _samples: MessageList<ProfileSample>,
    /// Latitude where the profile was calculated.
    pub _lat: f64,
    /// Longitude where the profile was calculated.
    pub _lon: f64,
}

impl Message for VerticalProfile {
    fn new() -> VerticalProfile {
        VerticalProfile {
            _header: Header::new(111),
            _parameter: Default::default(),
            _numSamples: Default::default(),
            _samples: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        111
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        111
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
        for m in &mut self._samples {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        for m in &mut self._samples {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        for m in &mut self._samples {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        for m in &mut self._samples {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        for m in &mut self._samples {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(111);
        self._parameter = Default::default();
        self._numSamples = Default::default();
        self._samples = Default::default();
        self._lat = Default::default();
        self._lon = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        18
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._samples);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._parameter);
        bfr.put_u8(self._numSamples);
        serialize_message_list!(bfr, self._samples);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._parameter = bfr.get_u8();
        self._numSamples = bfr.get_u8();
        self._samples = deserialize_message_list_as::<ProfileSample>(bfr)?;
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        Ok(())
    }
}
