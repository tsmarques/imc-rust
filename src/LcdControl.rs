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

use crate::Header::Header;
use crate::Message::*;

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Turn off display
    OP_TURN_OFF = 0,
    /// Turn on display
    OP_TURN_ON = 1,
    /// Clear display
    OP_CLEAR = 2,
    /// Write Line #0
    OP_WRITE0 = 3,
    /// Write Line #1
    OP_WRITE1 = 4,
}

/// Control LCD.
#[derive(Default)]
pub struct LcdControl {
    /// Message Header
    pub _header: Header,
    /// The LCD action to perform
    pub _op: u8,
    /// Text to be written (if defined write operation).
    pub _text: String,
}

impl Message for LcdControl {
    fn new() -> LcdControl {
        let msg = LcdControl {
            _header: Header::new(307),
            _op: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        307
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        307
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(307);
        self._op = Default::default();
        self._text = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._text);
        Ok(())
    }
}
