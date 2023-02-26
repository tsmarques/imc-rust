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

/// Airspeed along with airflow angles.
#[derive(Default, Clone)]
pub struct Airflow {
    /// Message Header
    pub _header: Header,
    /// Airspeed, the 2-norm of the relative velocity.
    pub _va: f32,
    /// Angle of attack.
    pub _aoa: f32,
    /// Sideslip angle.
    pub _ssa: f32,
}

impl Message for Airflow {
    fn new() -> Airflow {
        

        Airflow {
            _header: Header::new(363),
            _va: Default::default(),
            _aoa: Default::default(),
            _ssa: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        363
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        363
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
        self._header = Header::new(363);
        self._va = Default::default();
        self._aoa = Default::default();
        self._ssa = Default::default()
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
        bfr.put_f32_le(self._va);
        bfr.put_f32_le(self._aoa);
        bfr.put_f32_le(self._ssa);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._va = bfr.get_f32_le();
        self._aoa = bfr.get_f32_le();
        self._ssa = bfr.get_f32_le();
        Ok(())
    }
}
