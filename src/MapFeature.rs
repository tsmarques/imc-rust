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
use crate::MapPoint::MapPoint;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// FeatureType
#[allow(non_camel_case_types)]
pub enum FeatureTypeEnum {
    /// Point of Interest
    FTYPE_POI = 0,
    /// Filled Polygon
    FTYPE_FILLEDPOLY = 1,
    /// Countoured Polygon
    FTYPE_CONTOUREDPOLY = 2,
    /// Line
    FTYPE_LINE = 3,
    /// Transponder
    FTYPE_TRANSPONDER = 4,
    /// Start Location
    FTYPE_STARTLOC = 5,
    /// Home Reference
    FTYPE_HOMEREF = 6,
}

/// A feature to appear on the map
#[derive(Default, Clone)]
pub struct MapFeature {
    /// Message Header
    pub _header: Header,
    /// The unique identifier for this feature (used as the name for points of interest)
    pub _id: String,
    /// The type of feature
    pub _feature_type: u8,
    /// The red component of the color for this point
    pub _rgb_red: u8,
    /// The green component of the color for this point
    pub _rgb_green: u8,
    /// The blue component of the color for this point
    pub _rgb_blue: u8,
    /// The enclosing feature definition.
    pub _feature: MessageList<MapPoint>,
}

impl Message for MapFeature {
    fn new() -> MapFeature {
        

        MapFeature {
            _header: Header::new(603),
            _id: Default::default(),
            _feature_type: Default::default(),
            _rgb_red: Default::default(),
            _rgb_green: Default::default(),
            _rgb_blue: Default::default(),
            _feature: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        603
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        603
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
        for m in &mut self._feature {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._feature {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._feature {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._feature {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._feature {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(603);
        self._id = Default::default();
        self._feature_type = Default::default();
        self._rgb_red = Default::default();
        self._rgb_green = Default::default();
        self._rgb_blue = Default::default();
        self._feature = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._id.len() + 2;
        message_list_serialization_size!(dyn_size, self._feature);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._id.as_bytes());
        bfr.put_u8(self._feature_type);
        bfr.put_u8(self._rgb_red);
        bfr.put_u8(self._rgb_green);
        bfr.put_u8(self._rgb_blue);
        serialize_message_list!(bfr, self._feature);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._id);
        self._feature_type = bfr.get_u8();
        self._rgb_red = bfr.get_u8();
        self._rgb_green = bfr.get_u8();
        self._rgb_blue = bfr.get_u8();
        self._feature = deserialize_message_list_as::<MapPoint>(bfr)?;
        Ok(())
    }
}
