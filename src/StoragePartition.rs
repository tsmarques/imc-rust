//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Status
#[allow(non_camel_case_types)]
pub enum StatusEnum {
    /// Mounted
    SDST_MOUNTED = 0,
    /// Unmounted
    SDST_UNMOUNTED = 1,
    /// Unknown
    SDST_UNKWN = 2,
}

#[derive(Default)]
pub struct StoragePartition {
    /// Message Header
    pub _header: Header,
    /// Partition path, e.g. /dev/sdb1 or nvme0n1p1.
    pub _part_path: String,
    /// User defined label
    pub _label: String,
    /// Partition size in MiB.
    pub _size: u32,
    /// Used partition size, as a percentage.
    pub _used_size: f32,
    /// Text description of the filesystem type, e.g. ext4. It should match
    /// the real name of the type.
    pub _fstype: String,
    pub _status: u8,
}

impl Message for StoragePartition {
    fn new() -> StoragePartition {
        let msg = StoragePartition {
            _header: Header::new(109),
            _part_path: Default::default(),
            _label: Default::default(),
            _size: Default::default(),
            _used_size: Default::default(),
            _fstype: Default::default(),
            _status: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        109
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        109
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(109);
        self._part_path = Default::default();
        self._label = Default::default();
        self._size = Default::default();
        self._used_size = Default::default();
        self._fstype = Default::default();
        self._status = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._part_path.len() + 2;
        dyn_size += self._label.len() + 2;
        dyn_size += self._fstype.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._part_path.as_bytes());
        serialize_bytes!(bfr, self._label.as_bytes());
        bfr.put_u32_le(self._size);
        bfr.put_f32_le(self._used_size);
        serialize_bytes!(bfr, self._fstype.as_bytes());
        bfr.put_u8(self._status);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._part_path);
        deserialize_string!(bfr, self._label);
        self._size = bfr.get_u32_le();
        self._used_size = bfr.get_f32_le();
        deserialize_string!(bfr, self._fstype);
        self._status = bfr.get_u8();
        Ok(())
    }
}
