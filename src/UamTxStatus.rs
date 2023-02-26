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

/// Value
#[allow(non_camel_case_types)]
pub enum ValueEnum {
    /// Transmission Completed
    UTS_DONE = 0,
    /// Transmission Failed
    UTS_FAILED = 1,
    /// Transmission Canceled
    UTS_CANCELED = 2,
    /// Modem is busy
    UTS_BUSY = 3,
    /// Invalid address
    UTS_INV_ADDR = 4,
    /// In Progress
    UTS_IP = 5,
    /// Unsupported operation
    UTS_UNSUPPORTED = 6,
    /// Invalid transmission size
    UTS_INV_SIZE = 7,
    /// Message has been sent
    UTS_SENT = 8,
    /// Message has been acknowledged by the destination
    UTS_DELIVERED = 9,
}

/// This message shall be used by acoustic modem drivers to send updates
/// on the transmission status of data frames.
#[derive(Default, Clone)]
pub struct UamTxStatus {
    /// Message Header
    pub _header: Header,
    /// The sequence identifier of the frame transmission request.
    pub _seq: u16,
    /// Frame transmission status.
    pub _value: u8,
    /// Where applicable this field shall contain a human-readable message
    /// explaining the error.
    pub _error: String,
}

impl Message for UamTxStatus {
    fn new() -> UamTxStatus {
        

        UamTxStatus {
            _header: Header::new(816),
            _seq: Default::default(),
            _value: Default::default(),
            _error: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        816
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        816
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
        self._header = Header::new(816);
        self._seq = Default::default();
        self._value = Default::default();
        self._error = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        bfr.put_u8(self._value);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u16_le();
        self._value = bfr.get_u8();
        deserialize_string!(bfr, self._error);
        Ok(())
    }
}
