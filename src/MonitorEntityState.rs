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
// IMC XML MD5: 3ec4b61a1b487d356bfc62e124f22651                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Command
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Reset to defaults
    MES_RESET = 0,
    /// Enable Monitoring
    MES_ENABLE = 1,
    /// Disable Monitoring
    MES_DISABLE = 2,
    /// Enable Monitoring (exclusive - disables all others)
    MES_ENABLE_EXCLUSIVE = 3,
    /// Status Report
    MES_STATUS = 4,
}

/// Controls monitoring of entity states in the vehicle.
#[derive(Default)]
pub struct MonitorEntityState {
    /// Message Header
    pub _header: Header,
    /// Command.
    pub _command: u8,
    /// Comma separated list of entity names.
    pub _entities: String,
}

impl Message for MonitorEntityState {
    fn new() -> MonitorEntityState {
        let msg = MonitorEntityState {
            _header: Header::new(502),
            _command: Default::default(),
            _entities: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        502
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        502
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(502);
        self._command = Default::default();
        self._entities = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._entities.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._command);
        serialize_bytes!(bfr, self._entities.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._command = bfr.get_u8();
        deserialize_string!(bfr, self._entities);
        Ok(())
    }
}
