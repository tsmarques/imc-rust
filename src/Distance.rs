//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::BeamConfig::BeamConfig;
use crate::DeviceState::DeviceState;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Validity
#[allow(non_camel_case_types)]
pub enum ValidityEnum {
    /// Invalid
    DV_INVALID = 0,
    /// Valid
    DV_VALID = 1,
}

/// Distance measurement detected by the device.
#[derive(Default, Clone)]
pub struct Distance {
    /// Message Header
    pub _header: Header,
    /// Validity of the measurement.
    pub _validity: u8,
    /// Device Location in the system.
    pub _location: MessageList<DeviceState>,
    /// Beam configuration of the device.
    pub _beam_config: MessageList<BeamConfig>,
    /// Measured distance.
    pub _value: f32,
}

impl Message for Distance {
    fn new() -> Distance {
        let msg = Distance {
            _header: Header::new(262),
            _validity: Default::default(),
            _location: Default::default(),
            _beam_config: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        262
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        262
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._location {
            m.set_timestamp_secs(ts);
        }
        for m in &mut self._beam_config {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._location {
            m.set_source(src);
        }
        for m in &mut self._beam_config {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._location {
            m.set_source_ent(src_ent);
        }
        for m in &mut self._beam_config {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._location {
            m.set_destination(dst);
        }
        for m in &mut self._beam_config {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._location {
            m.set_destination_ent(dst_ent);
        }
        for m in &mut self._beam_config {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(262);
        self._validity = Default::default();
        self._location = Default::default();
        self._beam_config = Default::default();
        self._value = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._location);
        message_list_serialization_size!(dyn_size, self._beam_config);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._validity);
        serialize_message_list!(bfr, self._location);
        serialize_message_list!(bfr, self._beam_config);
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._validity = bfr.get_u8();
        self._location = deserialize_message_list_as::<DeviceState>(bfr)?;
        self._beam_config = deserialize_message_list_as::<BeamConfig>(bfr)?;
        self._value = bfr.get_f32_le();
        Ok(())
    }
}
