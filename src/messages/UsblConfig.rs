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
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header;
use crate::Message;
use crate::MessageList;
use crate::UsblModem;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Set LBL Configuration
    OP_SET_CFG = 0,
    /// Retrieve LBL Configuration
    OP_GET_CFG = 1,
    /// Reply to a GET command
    OP_CUR_CFG = 2,
}

/// Ultra-Short Base Line configuration.
#[derive(Default, Clone)]
pub struct UsblConfig {
    /// Message Header
    pub _header: Header,
    /// Used to define the type of the operation this message holds.
    pub _op: u8,
    /// A list of USBL modem configuration messages.
    pub _modems: MessageList<UsblModem>,
}

impl Message for UsblConfig {
    fn new() -> UsblConfig {
        UsblConfig {
            _header: Header::new(902),
            _op: Default::default(),
            _modems: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        902
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        902
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&self) -> &Header {
        &self._header
    }

    fn get_mut_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_mut_header()._timestamp = ts;
        for m in &mut self._modems {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        for m in &mut self._modems {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        for m in &mut self._modems {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        for m in &mut self._modems {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        for m in &mut self._modems {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(902);
        self._op = Default::default();
        self._modems = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._modems);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_message_list!(bfr, self._modems);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._modems = deserialize_message_list_as::<UsblModem>(bfr)?;
        Ok(())
    }
}
