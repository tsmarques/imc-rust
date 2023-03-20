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
use crate::HistoricData::HistoricData;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Request Type
#[allow(non_camel_case_types)]
pub enum RequestTypeEnum {
    /// Query
    HRTYPE_QUERY = 1,
    /// Reply
    HRTYPE_REPLY = 2,
    /// Clear
    HRTYPE_CLEAR = 3,
}

#[derive(Default, Clone)]
pub struct HistoricDataQuery {
    /// Message Header
    pub _header: Header,
    pub _req_id: u16,
    pub _type: u8,
    pub _max_size: u16,
    pub _data: Option<HistoricData>,
}

impl Message for HistoricDataQuery {
    fn new() -> HistoricDataQuery {
        HistoricDataQuery {
            _header: Header::new(187),
            _req_id: Default::default(),
            _type: Default::default(),
            _max_size: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        187
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        187
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
        if let Some(m) = &mut self._data {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        if let Some(m) = &mut self._data {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        if let Some(m) = &mut self._data {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        if let Some(m) = &mut self._data {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._data {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(187);
        self._req_id = Default::default();
        self._type = Default::default();
        self._max_size = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._data);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u8(self._type);
        bfr.put_u16_le(self._max_size);
        serialize_inline_message!(bfr, self._data);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._type = bfr.get_u8();
        self._max_size = bfr.get_u16_le();
        self._data = deserialize_inline_as::<HistoricData>(bfr).ok();
        Ok(())
    }
}
