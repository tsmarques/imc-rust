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
use crate::Header::Header;
use crate::Message::*;
use crate::Reference::Reference;
use crate::DUNE_IMC_CONST_NULL_ID;

/// State.
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Waiting for first reference.
    FR_WAIT = 1,
    /// Going towards received reference.
    FR_GOTO = 2,
    /// Loitering after arriving at the reference.
    FR_LOITER = 3,
    /// Hovering after arriving at the reference.
    FR_HOVER = 4,
    /// Moving in z after arriving at the target cylinder.
    FR_ELEVATOR = 5,
    /// Controlling system timed out.
    FR_TIMEOUT = 6,
}

/// Proximity.
#[allow(non_camel_case_types)]
pub mod ProximityBits {
    /// Far from the destination.
    pub const PROX_FAR: u32 = 0x01;
    /// Near in the horizontal plane.
    pub const PROX_XY_NEAR: u32 = 0x02;
    /// Near in the vertical plane.
    pub const PROX_Z_NEAR: u32 = 0x04;
    /// Unreachable in the horizontal plane.
    pub const PROX_XY_UNREACHABLE: u32 = 0x08;
    /// Unreachable in the vertical plane.
    pub const PROX_Z_UNREACHABLE: u32 = 0x10;
}

#[derive(Default)]
pub struct FollowRefState {
    /// Message Header.
    pub _header: Header,
    /// Controlling Source.
    pub _control_src: u16,
    /// Controlling Entity.
    pub _control_ent: u8,
    /// Reference.
    pub _reference: Option<Reference>,
    /// State.
    pub _state: u8,
    /// Proximity.
    pub _proximity: u8,
}

impl Message for FollowRefState {
    fn new() -> FollowRefState
    where
        Self: Sized,
    {
        let msg = FollowRefState {
            _header: Header::new(480),
            _control_src: Default::default(),
            _control_ent: Default::default(),
            _reference: Default::default(),
            _state: Default::default(),
            _proximity: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        480
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        480
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(480);
        self._control_src = Default::default();
        self._control_ent = Default::default();
        self._reference = Default::default();
        self._state = Default::default();
        self._proximity = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._reference);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._control_src);
        bfr.put_u8(self._control_ent);
        serialize_inline_message!(bfr, self._reference);
        bfr.put_u8(self._state);
        bfr.put_u8(self._proximity);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._control_src = bfr.get_u16_le();
        self._control_ent = bfr.get_u8();
        self._reference = deserialize_inline_as::<Reference>(bfr).ok();
        self._state = bfr.get_u8();
        self._proximity = bfr.get_u8();
        Ok(())
    }
}
