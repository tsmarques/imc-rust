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
use crate::MapFeature::MapFeature;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// This message represents a simple map that is transferred between CCU consoles (from Neptus to ACCU)
#[derive(Default, Clone)]
pub struct Map {
    /// Message Header
    pub _header: Header,
    /// The id of the map
    pub _id: String,
    /// A list of map features.
    pub _features: MessageList<MapFeature>,
}

impl Message for Map {
    fn new() -> Map {
        

        Map {
            _header: Header::new(602),
            _id: Default::default(),
            _features: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        602
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        602
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

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._features {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._features {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._features {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._features {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._features {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(602);
        self._id = Default::default();
        self._features = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._id.len() + 2;
        message_list_serialization_size!(dyn_size, self._features);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._id.as_bytes());
        serialize_message_list!(bfr, self._features);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._id);
        self._features = deserialize_message_list_as::<MapFeature>(bfr)?;
        Ok(())
    }
}
