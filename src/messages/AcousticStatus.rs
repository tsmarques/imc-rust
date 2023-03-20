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

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Abort
    TYPE_ABORT = 0,
    /// Range
    TYPE_RANGE = 1,
    /// Reverse Range
    TYPE_REVERSE_RANGE = 2,
    /// Message
    TYPE_MSG = 3,
    /// Raw
    TYPE_RAW = 4,
}

/// Status
#[allow(non_camel_case_types)]
pub enum StatusEnum {
    /// Queued
    STATUS_QUEUED = 0,
    /// In Progress
    STATUS_IN_PROGRESS = 1,
    /// Sent
    STATUS_SENT = 2,
    /// Range Received
    STATUS_RANGE_RECEIVED = 3,
    /// Delivered
    STATUS_DELIVERED = 4,
    /// Busy
    STATUS_BUSY = 100,
    /// Input Error
    STATUS_INPUT_FAILURE = 101,
    /// Error trying to send acoustic text
    STATUS_ERROR = 102,
    /// Message Type is not defined or is unsupported
    STATUS_UNSUPPORTED = 666,
}

/// Reply sent in response to a Acoustic Text sending request.
#[derive(Default, Clone)]
pub struct AcousticStatus {
    /// Message Header
    pub _header: Header,
    pub _req_id: u16,
    pub _type: u8,
    pub _status: u8,
    /// Status description.
    pub _info: String,
    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _range: f32,
}

impl Message for AcousticStatus {
    fn new() -> AcousticStatus {
        AcousticStatus {
            _header: Header::new(216),
            _req_id: Default::default(),
            _type: Default::default(),
            _status: Default::default(),
            _info: Default::default(),
            _range: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        216
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        216
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
        self._header = Header::new(216);
        self._req_id = Default::default();
        self._type = Default::default();
        self._status = Default::default();
        self._info = Default::default();
        self._range = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        8
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u8(self._type);
        bfr.put_u8(self._status);
        serialize_bytes!(bfr, self._info.as_bytes());
        bfr.put_f32_le(self._range);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._type = bfr.get_u8();
        self._status = bfr.get_u8();
        deserialize_string!(bfr, self._info);
        self._range = bfr.get_f32_le();
        Ok(())
    }
}
