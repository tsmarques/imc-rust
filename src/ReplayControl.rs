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

/// Operation.
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Start.
    ROP_START = 0,
    /// Stop.
    ROP_STOP = 1,
    /// Pause.
    ROP_PAUSE = 2,
    /// Resume.
    ROP_RESUME = 3,
}

/// Control replay of LSF logged data.
#[derive(Default)]
pub struct ReplayControl {
    /// Message Header.
    pub _header: Header,
    /// Operation.
    pub _op: u8,
    /// File To Replay.
    pub _file: String,
}

impl Message for ReplayControl {
    fn new() -> ReplayControl
    where
        Self: Sized,
    {
        let msg = ReplayControl {
            _header: Header::new(105),
            _op: Default::default(),
            _file: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        105
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        105
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(105);
        self._op = Default::default();
        self._file = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._file.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._file.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._file);
        Ok(())
    }
}
