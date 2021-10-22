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

/// This maneuver follows a direct command given by an external entity.
#[derive(Default)]
pub struct FollowCommand {
    /// Message Header
    pub _header: Header,
    /// The IMC identifier of the source system that is allowed to provide command to this maneuver.
    /// If the value ''0xFFFF'' is used, any system is allowed to command references.
    pub _control_src: u16,
    /// The entity identifier of the entity that is allowed to provide commands to this maneuver.
    /// If the value ''0xFF'' is used, any entity is allowed to command references.
    pub _control_ent: u8,
    /// The ammount of time, in seconds, after which the maneuver will be terminated if no new command has
    /// been received. In other words, the controlling entity should send command updates in shorter periods than
    /// 'timeout'.
    pub _timeout: f32,
}

impl Message for FollowCommand {
    fn new() -> FollowCommand {
        let msg = FollowCommand {
            _header: Header::new(496),
            _control_src: Default::default(),
            _control_ent: Default::default(),
            _timeout: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        496
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        496
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(496);
        self._control_src = Default::default();
        self._control_ent = Default::default();
        self._timeout = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        7
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._control_src);
        bfr.put_u8(self._control_ent);
        bfr.put_f32_le(self._timeout);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._control_src = bfr.get_u16_le();
        self._control_ent = bfr.get_u8();
        self._timeout = bfr.get_f32_le();
        Ok(())
    }
}
