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

use crate::Header::Header;
use crate::Message::*;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Idle
    FLEV_IDLE = 0,
    /// Ignition
    FLEV_IGNITION = 1,
    /// Lift Off
    FLEV_LIFTOFF = 2,
    /// Maximum Dynamic Pressure
    FLEV_MAX_Q = 3,
    /// Coasting
    FLEV_COASTING = 4,
    /// Apogee
    FLEV_APOGEE = 5,
    /// Recovery
    FLEV_RECOVERY = 6,
    /// Touchdown
    FLEV_TOUCHDOWN = 7,
}

/// Launch Vehicle flight events
#[derive(Default, Clone)]
pub struct FlightEvent {
    /// Message Header
    pub _header: Header,
    /// Flight Event
    pub _type: u8,
}

impl Message for FlightEvent {
    fn new() -> FlightEvent {
        FlightEvent {
            _header: Header::new(2011),
            _type: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2011
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2011
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

    fn clear(&mut self) {
        self._header = Header::new(2011);
        self._type = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        Ok(())
    }
}
