//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Type.
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request.
    SCTR_REQUEST = 0,
    /// Reply -- Success.
    SCTR_SUCCESS = 1,
    /// Reply -- Failure.
    SCTR_FAILURE = 2,
    /// Reply -- In Progress.
    SCTR_IN_PROGRESS = 3,
}

/// Operation.
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// List.
    SOP_LIST = 0,
    /// Mount.
    SOP_MOUNT = 1,
    /// Unmount.
    SOP_UMOUNT = 2,
    /// Format.
    SOP_FORMAT = 3,
}

/// Request/reply storage operations
#[derive(Default)]
pub struct StorageControl {
    /// Message Header.
    pub _header: Header,
    /// Type.
    pub _type: u8,
    /// Operation.
    pub _op: u8,
    /// Request ID.
    pub _request_id: u16,
    /// Device ID.
    pub _device_id: String,
    /// Complementary Information.
    pub _info: String,
}

impl Message for StorageControl {
    fn new() -> StorageControl {
        let msg = StorageControl {
            _header: Header::new(107),
            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _device_id: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        107
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        107
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(107);
        self._type = Default::default();
        self._op = Default::default();
        self._request_id = Default::default();
        self._device_id = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._device_id.len() + 2;
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._op);
        bfr.put_u16_le(self._request_id);
        serialize_bytes!(bfr, self._device_id.as_bytes());
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._op = bfr.get_u8();
        self._request_id = bfr.get_u16_le();
        deserialize_string!(bfr, self._device_id);
        deserialize_string!(bfr, self._info);
        Ok(())
    }
}
