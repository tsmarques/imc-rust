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

/// Request SMS Text sending.
#[derive(Default, Clone)]
pub struct SmsRequest {
    /// Message Header
    pub _header: Header,
    pub _req_id: u16,
    /// Recipient identifier (number or name).
    pub _destination: String,
    /// Period of time to send message (in seconds).
    pub _timeout: f64,
    pub _sms_text: String,
}

impl Message for SmsRequest {
    fn new() -> SmsRequest {
        SmsRequest {
            _header: Header::new(517),
            _req_id: Default::default(),
            _destination: Default::default(),
            _timeout: Default::default(),
            _sms_text: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        517
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        517
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
        self._header = Header::new(517);
        self._req_id = Default::default();
        self._destination = Default::default();
        self._timeout = Default::default();
        self._sms_text = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        10
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._destination.len() + 2;
        dyn_size += self._sms_text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        serialize_bytes!(bfr, self._destination.as_bytes());
        bfr.put_f64_le(self._timeout);
        serialize_bytes!(bfr, self._sms_text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        deserialize_string!(bfr, self._destination);
        self._timeout = bfr.get_f64_le();
        deserialize_string!(bfr, self._sms_text);
        Ok(())
    }
}
