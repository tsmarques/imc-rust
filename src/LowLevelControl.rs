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
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Low level maneuver that sends a (heading, roll, speed, ...)
/// reference to a controller of the vehicle and then optionally
/// lingers for some time.
#[derive(Default, Clone)]
pub struct LowLevelControl {
    /// Message Header
    pub _header: Header,
    /// Control command: can be of type DesiredZ, DesiredHeading,
    /// DesiredRoll, DesiredPitch, DesiredSpeed, DesiredThrottle or DesiredPath.
    pub _control: Option<Box<dyn Message>>,
    /// Duration of the control.
    pub _duration: u16,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for LowLevelControl {
    fn new() -> LowLevelControl {
        

        LowLevelControl {
            _header: Header::new(455),
            _control: Default::default(),
            _duration: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        455
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        455
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

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        if let Some(m) = &mut self._control {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._control {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._control {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._control {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._control {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(455);
        self._control = Default::default();
        self._duration = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._control);
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_inline_message!(bfr, self._control);
        bfr.put_u16_le(self._duration);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._control = deserialize_inline(bfr).ok();
        self._duration = bfr.get_u16_le();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
