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
use crate::MessageList;
use crate::StoragePartition::StoragePartition;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Storage device information (e.g disk, usb flash, etc). NOTE: This is different from a partition
#[derive(Default)]
pub struct StorageDevice {
    /// Message Header.
    pub _header: Header,
    /// Model.
    pub _device_model: String,
    /// Size.
    pub _size: u32,
    /// Path.
    pub _path: String,
    /// Partition Type.
    pub _ptype: String,
    /// Partitions.
    pub _partitions: MessageList<StoragePartition>,
    /// Main Device.
    pub _is_main_device: u8,
}

impl Message for StorageDevice {
    fn new() -> StorageDevice {
        let msg = StorageDevice {
            _header: Header::new(108),
            _device_model: Default::default(),
            _size: Default::default(),
            _path: Default::default(),
            _ptype: Default::default(),
            _partitions: Default::default(),
            _is_main_device: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        108
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        108
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._partitions {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._partitions {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._partitions {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._partitions {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._partitions {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(108);
        self._device_model = Default::default();
        self._size = Default::default();
        self._path = Default::default();
        self._ptype = Default::default();
        self._partitions = Default::default();
        self._is_main_device = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._device_model.len() + 2;
        dyn_size += self._path.len() + 2;
        dyn_size += self._ptype.len() + 2;
        message_list_serialization_size!(dyn_size, self._partitions);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._device_model.as_bytes());
        bfr.put_u32_le(self._size);
        serialize_bytes!(bfr, self._path.as_bytes());
        serialize_bytes!(bfr, self._ptype.as_bytes());
        serialize_message_list!(bfr, self._partitions);
        bfr.put_u8(self._is_main_device);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._device_model);
        self._size = bfr.get_u32_le();
        deserialize_string!(bfr, self._path);
        deserialize_string!(bfr, self._ptype);
        self._partitions = deserialize_message_list_as::<StoragePartition>(bfr)?;
        self._is_main_device = bfr.get_u8();
        Ok(())
    }
}
