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

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Direction
#[allow(non_camel_case_types)]
pub enum DirectionEnum {
    /// Let the vehicle decide
    DIR_AUTO = 0,
    /// Attempt to move forward
    DIR_FORWARD = 1,
    /// Attempt to move backward
    DIR_BACKWARD = 2,
}

/// A "Dislodge" is a maneuver ordering the vehicle to attempt a
/// series of thruster operations that will hopefully get it
/// unstuck from an entangled condition.
/// Parameters are RPMs for the motor when attempting dislodge and
/// and a flag specifying whether the thrust burst should be attempted
/// forward, backward or auto (letting the vehicle decide).
#[derive(Default, Clone)]
pub struct Dislodge {
    /// Message Header
    pub _header: Header,
    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,
    /// Maneuver RPM reference.
    pub _rpm: f32,
    /// Direction to which the vehicle should attempt to unstuck.
    pub _direction: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Dislodge {
    fn new() -> Dislodge {
        

        Dislodge {
            _header: Header::new(483),
            _timeout: Default::default(),
            _rpm: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        483
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        483
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(483);
        self._timeout = Default::default();
        self._rpm = Default::default();
        self._direction = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        7
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_f32_le(self._rpm);
        bfr.put_u8(self._direction);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        self._rpm = bfr.get_f32_le();
        self._direction = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
