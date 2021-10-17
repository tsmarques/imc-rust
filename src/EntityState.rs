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
use crate::Header::Header;
use crate::Message::*;

/// State.
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Bootstrapping.
    ESTA_BOOT = 0,
    /// Normal Operation.
    ESTA_NORMAL = 1,
    /// Fault.
    ESTA_FAULT = 2,
    /// Error.
    ESTA_ERROR = 3,
    /// Failure.
    ESTA_FAILURE = 4,
}

/// Flags.
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Human Intervention Required.
    pub const EFLA_HUMAN_INTERVENTION: u32 = 0x01;
}

/// State reported by an entity in the vehicle. The source entity is
/// identified in the message header.
#[derive(Default)]
pub struct EntityState {
    /// Message Header.
    pub _header: Header,
    /// State.
    pub _state: u8,
    /// Flags.
    pub _flags: u8,
    /// Complementary description.
    pub _description: String,
}

impl Message for EntityState {
    fn new() -> EntityState
    where
        Self: Sized,
    {
        let msg = EntityState {
            _header: Header::new(1),
            _state: Default::default(),
            _flags: Default::default(),
            _description: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        1
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        1
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(1);
        self._state = Default::default();
        self._flags = Default::default();
        self._description = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._description.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._description.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        self._flags = bfr.get_u8();
        deserialize_string!(bfr, self._description);
        Ok(())
    }
}
