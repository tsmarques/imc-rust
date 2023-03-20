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
use crate::DUNE_IMC_CONST_NULL_ID;

#[derive(Default, Clone)]
pub struct HistoricSample {
    /// Message Header
    pub _header: Header,
    /// The IMC identifier of the system that produced this sample.
    pub _sys_id: u16,
    /// The priority for this data sample. Default priority is 0. Samples with
    /// higher priorities will *always* be transmitted before samples with lower
    /// priorities. Samples with -127 priority will not be transmitted but just
    /// logged to disk locally.
    pub _priority: i8,
    /// Northing offsets relative to base latitude / longitude expressed in the enclosing `HistoricData` message.
    pub _x: i16,
    /// Easting offsets relative to base latitude / longitude expressed in the enclosing `HistoricData` message.
    pub _y: i16,
    /// Altitude / depth offsets relative to sea level expressed in decimeters.
    /// Negative values mean depth and positive values mean altitude.
    pub _z: i16,
    /// Time offset in seconds relative to the base time expressed in the enclosing `HistoricData` message.
    pub _t: i16,
    pub _sample: Option<Box<dyn Message>>,
}

impl Message for HistoricSample {
    fn new() -> HistoricSample {
        HistoricSample {
            _header: Header::new(186),
            _sys_id: Default::default(),
            _priority: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _t: Default::default(),
            _sample: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        186
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        186
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
        if let Some(m) = &mut self._sample {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        if let Some(m) = &mut self._sample {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        if let Some(m) = &mut self._sample {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        if let Some(m) = &mut self._sample {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._sample {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(186);
        self._sys_id = Default::default();
        self._priority = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._t = Default::default();
        self._sample = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        11
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._sample);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._sys_id);
        bfr.put_i8(self._priority);
        bfr.put_i16_le(self._x);
        bfr.put_i16_le(self._y);
        bfr.put_i16_le(self._z);
        bfr.put_i16_le(self._t);
        serialize_inline_message!(bfr, self._sample);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._sys_id = bfr.get_u16_le();
        self._priority = bfr.get_i8();
        self._x = bfr.get_i16_le();
        self._y = bfr.get_i16_le();
        self._z = bfr.get_i16_le();
        self._t = bfr.get_i16_le();
        self._sample = deserialize_inline(bfr).ok();
        Ok(())
    }
}
