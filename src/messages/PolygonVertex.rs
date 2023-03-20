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

use crate::Header;
use crate::Message;

/// This message is used to store the various polygon vertices for
/// CoverArea maneuvers.
#[derive(Default, Clone)]
pub struct PolygonVertex {
    /// Message Header
    pub _header: Header,
    /// WGS-84 Latitude for start point.
    pub _lat: f64,
    /// WGS-84 Longitude for start point.
    pub _lon: f64,
}

impl Message for PolygonVertex {
    fn new() -> PolygonVertex {
        PolygonVertex {
            _header: Header::new(474),
            _lat: Default::default(),
            _lon: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        474
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        474
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
        self._header = Header::new(474);
        self._lat = Default::default();
        self._lon = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        Ok(())
    }
}
