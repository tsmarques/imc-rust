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
use bytes::{Buf, BufMut};

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Control Operation.
#[allow(non_camel_case_types)]
pub enum ControlOperationEnum {
    /// Store.
    COP_STORE = 0,
    /// Load.
    COP_LOAD = 1,
    /// Clear.
    COP_CLEAR = 2,
    /// Copy Snapshot.
    COP_COPY = 3,
    /// Snapshot Copy Complete.
    COP_COPY_COMPLETE = 4,
}

/// Control caching of messages to persistent storage.
#[derive(Default)]
pub struct CacheControl {
    /// Message Header.
    pub _header: Header,
    /// Control Operation.
    pub _op: u8,
    /// Snapshot destination.
    pub _snapshot: String,
    /// Message.
    pub _message: Option<Box<dyn Message>>,
}

impl Message for CacheControl {
    fn new() -> CacheControl
    where
        Self: Sized,
    {
        let msg = CacheControl {
            _header: Header::new(101),
            _op: Default::default(),
            _snapshot: Default::default(),
            _message: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        101
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        101
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
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
