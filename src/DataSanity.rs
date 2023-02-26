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

/// Sanity
#[allow(non_camel_case_types)]
pub enum SanityEnum {
    /// Sane
    DS_SANE = 0,
    /// Not Sane
    DS_NOT_SANE = 1,
}

/// Report sanity or lack of it in the data output by a sensor.
#[derive(Default, Clone)]
pub struct DataSanity {
    /// Message Header
    pub _header: Header,
    /// Whether the data is sane or not sane.
    pub _sane: u8,
}

impl Message for DataSanity {
    fn new() -> DataSanity {
        

        DataSanity {
            _header: Header::new(284),
            _sane: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        284
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        284
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
        self._header = Header::new(284);
        self._sane = Default::default()
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
        bfr.put_u8(self._sane);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._sane = bfr.get_u8();
        Ok(())
    }
}
