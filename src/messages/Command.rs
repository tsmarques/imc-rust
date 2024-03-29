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

use crate::Header;
use crate::Message;

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Use Speed Reference in meters per second
    pub const FLAG_SPEED_METERS_PS: u32 = 0x01;
    /// Use Speed Reference in revolutions per minute
    pub const FLAG_SPEED_RPM: u32 = 0x02;
    /// Use Z Reference as depth
    pub const FLAG_DEPTH: u32 = 0x04;
    /// Use Z Reference as altitude
    pub const FLAG_ALTITUDE: u32 = 0x08;
    /// Use Heading Reference
    pub const FLAG_HEADING: u32 = 0x10;
    /// Use Heading Rate Reference
    pub const FLAG_HEADING_RATE: u32 = 0x20;
    /// Flag Maneuver Completion
    pub const FLAG_MANDONE: u32 = 0x80;
}

/// This message must be sent by an external entity to provide command references to a system
/// running a "Follow Command Maneuver". If no Command messages are transmitted, the system
/// will terminate maneuver.
#[derive(Default, Clone)]
pub struct Command {
    /// Message Header
    pub _header: Header,
    pub _flags: u8,
    /// The value of the desired speed, in the scale specified by the
    /// "flags" field.
    pub _speed: f32,
    /// The value of the desired z reference in meters.
    pub _z: f32,
    /// The value of the desired heading angle, relative to true  north, in radians,
    /// or, the value of the desired heading rate angle, in radians.
    pub _heading: f32,
}

impl Message for Command {
    fn new() -> Command {
        

        Command {
            _header: Header::new(497),
            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _heading: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        497
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        497
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
        self._header = Header::new(497);
        self._flags = Default::default();
        self._speed = Default::default();
        self._z = Default::default();
        self._heading = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        13
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._flags);
        bfr.put_f32_le(self._speed);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._heading);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._flags = bfr.get_u8();
        self._speed = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._heading = bfr.get_f32_le();
        Ok(())
    }
}
