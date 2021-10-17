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

/// Operation.
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Abort.
    AOP_ABORT = 0,
    /// Abort in Progress.
    AOP_ABORT_IP = 1,
    /// Abort Timeout.
    AOP_ABORT_TIMEOUT = 2,
    /// Abort Acknowledged.
    AOP_ABORT_ACKED = 3,
    /// Range Request.
    AOP_RANGE = 4,
    /// Range in Progress.
    AOP_RANGE_IP = 5,
    /// Range Timeout.
    AOP_RANGE_TIMEOUT = 6,
    /// Range Received.
    AOP_RANGE_RECVED = 7,
    /// Modem is Busy.
    AOP_BUSY = 8,
    /// Unsupported operation.
    AOP_UNSUPPORTED = 9,
    /// Transducer Not Detected.
    AOP_NO_TXD = 10,
    /// Send Message.
    AOP_MSG = 11,
    /// Message Send -- Queued.
    AOP_MSG_QUEUED = 12,
    /// Message Send -- In progress.
    AOP_MSG_IP = 13,
    /// Message Send -- Done.
    AOP_MSG_DONE = 14,
    /// Message Send -- Failure.
    AOP_MSG_FAILURE = 15,
    /// Send Short Message.
    AOP_MSG_SHORT = 16,
}

/// Acoustic operation.
#[derive(Default)]
pub struct AcousticOperation {
    /// Message Header.
    pub _header: Header,
    /// Operation.
    pub _op: u8,
    /// System.
    pub _system: String,
    /// Range.
    pub _range: f32,
    /// Message To Send.
    pub _msg: Option<Box<dyn Message>>,
}

impl Message for AcousticOperation {
    fn new() -> AcousticOperation
    where
        Self: Sized,
    {
        let msg = AcousticOperation {
            _header: Header::new(211),
            _op: Default::default(),
            _system: Default::default(),
            _range: Default::default(),
            _msg: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        211
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        211
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(211);
        self._op = Default::default();
        self._system = Default::default();
        self._range = Default::default();
        self._msg = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._system.len() + 2;
        inline_message_serialization_size!(dyn_size, self._msg);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._system.as_bytes());
        bfr.put_f32_le(self._range);
        serialize_inline_message!(bfr, self._msg);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._system);
        self._range = bfr.get_f32_le();
        self._msg = deserialize_inline(bfr).ok();
        Ok(())
    }
}
