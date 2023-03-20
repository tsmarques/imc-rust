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

/// Status
#[allow(non_camel_case_types)]
pub enum StatusEnum {
    /// In progress
    TSTAT_IN_PROGRESS = 0,
    /// Sent
    TSTAT_SENT = 1,
    /// Delivered
    TSTAT_DELIVERED = 51,
    /// Delivery is unknown
    TSTAT_MAYBE_DELIVERED = 52,
    /// Range received
    TSTAT_RANGE_RECEIVED = 60,
    /// Input Error
    TSTAT_INPUT_FAILURE = 101,
    /// Temporary Error
    TSTAT_TEMPORARY_FAILURE = 102,
    /// Permanent Failure
    TSTAT_PERMANENT_FAILURE = 103,
}

/// Reply sent in response to a communications request.
#[derive(Default, Clone)]
pub struct TransmissionStatus {
    /// Message Header
    pub _header: Header,
    pub _req_id: u16,
    pub _status: u8,
    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _range: f32,
    pub _info: String,
}

impl Message for TransmissionStatus {
    fn new() -> TransmissionStatus {
        TransmissionStatus {
            _header: Header::new(516),
            _req_id: Default::default(),
            _status: Default::default(),
            _range: Default::default(),
            _info: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        516
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        516
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
        self._header = Header::new(516);
        self._req_id = Default::default();
        self._status = Default::default();
        self._range = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        7
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u8(self._status);
        bfr.put_f32_le(self._range);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._status = bfr.get_u8();
        self._range = bfr.get_f32_le();
        deserialize_string!(bfr, self._info);
        Ok(())
    }
}
