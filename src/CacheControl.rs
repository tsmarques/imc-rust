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
// IMC XML MD5: 3ec4b61a1b487d356bfc62e124f22651                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Control Operation
#[allow(non_camel_case_types)]
pub enum ControlOperationEnum {
    /// Store
    COP_STORE = 0,
    /// Load
    COP_LOAD = 1,
    /// Clear
    COP_CLEAR = 2,
    /// Copy Snapshot
    COP_COPY = 3,
    /// Snapshot Copy Complete
    COP_COPY_COMPLETE = 4,
}

/// Control caching of messages to persistent storage.
#[derive(Default)]
pub struct CacheControl {
    /// Message Header
    pub _header: Header,
    /// Operation to perform.
    pub _op: u8,
    /// Destination for the cache snapshot file.
    pub _snapshot: String,
    /// Message to store.
    pub _message: Option<Box<dyn Message>>,
}

impl Message for CacheControl {
    fn new() -> CacheControl {
        let msg = CacheControl {
            _header: Header::new(101),
            _op: Default::default(),
            _snapshot: Default::default(),
            _message: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        101
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        101
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        if let Some(m) = &mut self._message {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._message {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._message {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._message {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._message {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(101);
        self._op = Default::default();
        self._snapshot = Default::default();
        self._message = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._snapshot.len() + 2;
        inline_message_serialization_size!(dyn_size, self._message);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._snapshot.as_bytes());
        serialize_inline_message!(bfr, self._message);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._snapshot);
        self._message = deserialize_inline(bfr).ok();
        Ok(())
    }
}
