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

/// Control Operation.
#[allow(non_camel_case_types)]
pub enum ControlOperationEnum {
    /// Request Start of Logging.
    COP_REQUEST_START = 0,
    /// Logging Started.
    COP_STARTED = 1,
    /// Request Logging Stop.
    COP_REQUEST_STOP = 2,
    /// Logging Stopped.
    COP_STOPPED = 3,
    /// Request Current Log Name.
    COP_REQUEST_CURRENT_NAME = 4,
    /// Current Log Name.
    COP_CURRENT_NAME = 5,
}

/// Control logging of messages to persistent storage.
#[derive(Default)]
pub struct LoggingControl {
    /// Message Header.
    pub _header: Header,
    /// Control Operation.
    pub _op: u8,
    /// Log Label / Path.
    pub _name: String,
}

impl Message for LoggingControl {
    fn new() -> LoggingControl {
        let msg = LoggingControl {
            _header: Header::new(102),
            _op: Default::default(),
            _name: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        102
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        102
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(102);
        self._op = Default::default();
        self._name = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._name.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._name);
        Ok(())
    }
}
