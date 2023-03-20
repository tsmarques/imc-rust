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

/// This message is used to store historic (transmitted afterwards) telemetry information.
#[derive(Default, Clone)]
pub struct HistoricTelemetry {
    /// Message Header
    pub _header: Header,
    pub _altitude: f32,
    /// Roll encoded as α.(65535/(2.π))
    pub _roll: u16,
    /// Pitch encoded as α.(65535/(2.π))
    pub _pitch: u16,
    /// Yaw encoded as α.(65535/(2.π))
    pub _yaw: u16,
    pub _speed: i16,
}

impl Message for HistoricTelemetry {
    fn new() -> HistoricTelemetry {
        HistoricTelemetry {
            _header: Header::new(108),
            _altitude: Default::default(),
            _roll: Default::default(),
            _pitch: Default::default(),
            _yaw: Default::default(),
            _speed: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        108
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        108
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
        self._header = Header::new(108);
        self._altitude = Default::default();
        self._roll = Default::default();
        self._pitch = Default::default();
        self._yaw = Default::default();
        self._speed = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        12
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._altitude);
        bfr.put_u16_le(self._roll);
        bfr.put_u16_le(self._pitch);
        bfr.put_u16_le(self._yaw);
        bfr.put_i16_le(self._speed);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._altitude = bfr.get_f32_le();
        self._roll = bfr.get_u16_le();
        self._pitch = bfr.get_u16_le();
        self._yaw = bfr.get_u16_le();
        self._speed = bfr.get_i16_le();
        Ok(())
    }
}
