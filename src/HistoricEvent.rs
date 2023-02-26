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

/// Event Type
#[allow(non_camel_case_types)]
pub enum EventTypeEnum {
    /// Information
    EVTYPE_INFO = 0,
    /// Error
    EVTYPE_ERROR = 1,
}

/// This message is used to store historic event log entries.
#[derive(Default, Clone)]
pub struct HistoricEvent {
    /// Message Header
    pub _header: Header,
    pub _text: String,
    /// Type of event.
    pub _type: u8,
}

impl Message for HistoricEvent {
    fn new() -> HistoricEvent {
        

        HistoricEvent {
            _header: Header::new(110),
            _text: Default::default(),
            _type: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        110
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        110
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
        self._header = Header::new(110);
        self._text = Default::default();
        self._type = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._text.as_bytes());
        bfr.put_u8(self._type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._text);
        self._type = bfr.get_u8();
        Ok(())
    }
}
