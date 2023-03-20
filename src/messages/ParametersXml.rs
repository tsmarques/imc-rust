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

use crate::Header;
use crate::Message;

/// Message containing the parameters XML of the source system.
#[derive(Default, Clone)]
pub struct ParametersXml {
    /// Message Header
    pub _header: Header,
    /// The locale used to produce this parameters XML.
    pub _locale: String,
    /// The parameters XML file compressed using the GNU zip (gzip) format.
    pub _config: Vec<u8>,
}

impl Message for ParametersXml {
    fn new() -> ParametersXml {
        ParametersXml {
            _header: Header::new(893),
            _locale: Default::default(),
            _config: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        893
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        893
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
        self._header = Header::new(893);
        self._locale = Default::default();
        self._config = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._locale.len() + 2;
        dyn_size += self._config.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._locale.as_bytes());
        serialize_bytes!(bfr, self._config.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._locale);
        deserialize_bytes!(bfr, self._config);
        Ok(())
    }
}
