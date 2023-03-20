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
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Abort
    TYPE_ABORT = 0,
    /// Range
    TYPE_RANGE = 1,
    /// Reverse Range
    TYPE_REVERSE_RANGE = 2,
    /// Message
    TYPE_MSG = 3,
    /// Raw
    TYPE_RAW = 4,
}

/// Request Acoustic sending.
#[derive(Default, Clone)]
pub struct AcousticRequest {
    /// Message Header
    pub _header: Header,
    pub _req_id: u16,
    /// The name of the system where to send this message.
    pub _destination: String,
    /// Period of time to send message (in seconds).
    pub _timeout: f64,
    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _range: f32,
    pub _type: u8,
    /// Argument for message send ('MSG') or ('RAW') but in this case expects DevDataBinary message requests.
    pub _msg: Option<Box<dyn Message>>,
}

impl Message for AcousticRequest {
    fn new() -> AcousticRequest {
        AcousticRequest {
            _header: Header::new(215),
            _req_id: Default::default(),
            _destination: Default::default(),
            _timeout: Default::default(),
            _range: Default::default(),
            _type: Default::default(),
            _msg: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        215
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        215
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
        if let Some(m) = &mut self._msg {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        if let Some(m) = &mut self._msg {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        if let Some(m) = &mut self._msg {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        if let Some(m) = &mut self._msg {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._msg {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(215);
        self._req_id = Default::default();
        self._destination = Default::default();
        self._timeout = Default::default();
        self._range = Default::default();
        self._type = Default::default();
        self._msg = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        15
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._destination.len() + 2;
        inline_message_serialization_size!(dyn_size, self._msg);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        serialize_bytes!(bfr, self._destination.as_bytes());
        bfr.put_f64_le(self._timeout);
        bfr.put_f32_le(self._range);
        bfr.put_u8(self._type);
        serialize_inline_message!(bfr, self._msg);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        deserialize_string!(bfr, self._destination);
        self._timeout = bfr.get_f64_le();
        self._range = bfr.get_f32_le();
        self._type = bfr.get_u8();
        self._msg = deserialize_inline(bfr).ok();
        Ok(())
    }
}
