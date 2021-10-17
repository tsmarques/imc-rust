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
use bytes::{Buf, BufMut};

use crate::packet::ImcError;
use crate::packet::*;
use crate::BeamConfig::BeamConfig;
use crate::DeviceState::DeviceState;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Validity.
#[allow(non_camel_case_types)]
pub enum ValidityEnum {
    /// Invalid.
    DV_INVALID = 0,
    /// Valid.
    DV_VALID = 1,
}

/// Distance measurement detected by the device.
#[derive(Default)]
pub struct Distance {
    /// Message Header.
    pub _header: Header,
    /// Validity.
    pub _validity: u8,
    /// Location.
    pub _location: MessageList<DeviceState>,
    /// Beam Configuration.
    pub _beam_config: MessageList<BeamConfig>,
    /// Measured Distance.
    pub _value: f32,
}

impl Message for Distance {
    fn new() -> Distance
    where
        Self: Sized,
    {
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
    fn static_id() -> u16
    where
        Self: Sized,
    {
        262
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        262
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
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
