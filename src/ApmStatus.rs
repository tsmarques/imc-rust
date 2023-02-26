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

/// Severity
#[allow(non_camel_case_types)]
pub enum SeverityEnum {
    /// Emergency
    APM_EMERGENCY = 0,
    /// Alert
    APM_ALERT = 1,
    /// Critical
    APM_CRITICAL = 2,
    /// Error
    APM_ERROR = 3,
    /// Warning
    APM_WARNING = 4,
    /// Notice
    APM_NOTICE = 5,
    /// Info
    APM_INFO = 6,
    /// Debug
    APM_DEBUG = 7,
}

/// StatusText message from ardupilot.
#[derive(Default, Clone)]
pub struct ApmStatus {
    /// Message Header
    pub _header: Header,
    /// Severity of status.
    pub _severity: u8,
    /// Status text message.
    pub _text: String,
}

impl Message for ApmStatus {
    fn new() -> ApmStatus {
        

        ApmStatus {
            _header: Header::new(906),
            _severity: Default::default(),
            _text: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        906
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        906
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
        self._header = Header::new(906);
        self._severity = Default::default();
        self._text = Default::default()
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
        bfr.put_u8(self._severity);
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._severity = bfr.get_u8();
        deserialize_string!(bfr, self._text);
        Ok(())
    }
}
