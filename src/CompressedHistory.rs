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

use crate::Header::Header;
use crate::Message::*;

/// This message holds a list of inline data samples produced by one or more vehicles in the past.
/// It is used to transfer data over disruption tolerant networks.
#[derive(Default, Clone)]
pub struct CompressedHistory {
    /// Message Header
    pub _header: Header,
    /// All data sent inside this message will have offsets relative to this latitude.
    pub _base_lat: f32,
    /// All data sent inside this message will have offsets relative to this longitude.
    pub _base_lon: f32,
    /// All data sent inside this message will use this time as the origin (0).
    pub _base_time: f32,
    /// A message-list of HistoricSample messages compressed with GZip algorithm.
    pub _data: Vec<u8>,
}

impl Message for CompressedHistory {
    fn new() -> CompressedHistory {
        CompressedHistory {
            _header: Header::new(185),
            _base_lat: Default::default(),
            _base_lon: Default::default(),
            _base_time: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        185
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        185
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

    fn clear(&mut self) {
        self._header = Header::new(185);
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
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._base_lat);
        bfr.put_f32_le(self._base_lon);
        bfr.put_f32_le(self._base_time);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._base_lat = bfr.get_f32_le();
        self._base_lon = bfr.get_f32_le();
        self._base_time = bfr.get_f32_le();
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
