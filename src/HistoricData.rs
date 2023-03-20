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
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// This message holds a list of inline data samples produced by one or more vehicles in the past.
/// It is used to transfer data over disruption tolerant networks.
#[derive(Default, Clone)]
pub struct HistoricData {
    /// Message Header
    pub _header: Header,
    /// All data sent inside this message will have offsets relative to this latitude.
    pub _base_lat: f32,
    /// All data sent inside this message will have offsets relative to this longitude.
    pub _base_lon: f32,
    /// All data sent inside this message will use this time as the origin (0).
    pub _base_time: f32,
    pub _data: MessageList<Box<dyn Message>>,
}

impl Message for HistoricData {
    fn new() -> HistoricData {
        HistoricData {
            _header: Header::new(184),
            _base_lat: Default::default(),
            _base_lon: Default::default(),
            _base_time: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        184
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        184
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
        for m in &mut self._data {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        for m in &mut self._data {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        for m in &mut self._data {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        for m in &mut self._data {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        for m in &mut self._data {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(184);
        self._base_lat = Default::default();
        self._base_lon = Default::default();
        self._base_time = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        12
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._data);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._base_lat);
        bfr.put_f32_le(self._base_lon);
        bfr.put_f32_le(self._base_time);
        serialize_message_list!(bfr, self._data);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._base_lat = bfr.get_f32_le();
        self._base_lon = bfr.get_f32_le();
        self._base_time = bfr.get_f32_le();
        self._data = deserialize_message_list(bfr)?;
        Ok(())
    }
}
