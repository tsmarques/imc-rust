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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Turn Off
    PCC_OP_TURN_OFF = 0,
    /// Turn On
    PCC_OP_TURN_ON = 1,
    /// Toggle
    PCC_OP_TOGGLE = 2,
    /// Schedule Turn On
    PCC_OP_SCHED_ON = 3,
    /// Schedule Turn Off
    PCC_OP_SCHED_OFF = 4,
    /// Reset Schedules
    PCC_OP_SCHED_RESET = 5,
    /// Save Current State
    PCC_OP_SAVE = 6,
}

/// This message allows controlling power channels.
#[derive(Default, Clone)]
pub struct PowerChannelControl {
    /// Message Header
    pub _header: Header,
    /// The name of the power channel.
    pub _name: String,
    /// Operation to perform.
    pub _op: u8,
    /// Scheduled time of operation.
    pub _sched_time: f64,
}

impl Message for PowerChannelControl {
    fn new() -> PowerChannelControl {
        

        PowerChannelControl {
            _header: Header::new(309),
            _name: Default::default(),
            _op: Default::default(),
            _sched_time: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        309
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        309
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
        self._header = Header::new(309);
        self._name = Default::default();
        self._op = Default::default();
        self._sched_time = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._op);
        bfr.put_f64_le(self._sched_time);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);
        self._op = bfr.get_u8();
        self._sched_time = bfr.get_f64_le();
        Ok(())
    }
}
