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

/// Medium
#[allow(non_camel_case_types)]
pub enum MediumEnum {
    /// WiFi
    RM_WIFI = 1,
    /// Satellite
    RM_SATELLITE = 2,
    /// Acoustic
    RM_ACOUSTIC = 3,
    /// SMS
    RM_SMS = 4,
}

/// This message is represents an Asset position / status.
#[derive(Default, Clone)]
pub struct AssetReport {
    /// Message Header
    pub _header: Header,
    /// The human readable name of the asset.
    pub _name: String,
    /// Time in seconds since epoch, for the generation instant.
    pub _report_time: f64,
    pub _medium: u8,
    pub _lat: f64,
    pub _lon: f64,
    pub _depth: f32,
    pub _alt: f32,
    pub _sog: f32,
    pub _cog: f32,
    pub _msgs: MessageList<Box<dyn Message>>,
}

impl Message for AssetReport {
    fn new() -> AssetReport {
        

        AssetReport {
            _header: Header::new(525),
            _name: Default::default(),
            _report_time: Default::default(),
            _medium: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _depth: Default::default(),
            _alt: Default::default(),
            _sog: Default::default(),
            _cog: Default::default(),
            _msgs: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        525
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        525
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
        for m in &mut self._msgs {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._msgs {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._msgs {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._msgs {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._msgs {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(525);
        self._name = Default::default();
        self._report_time = Default::default();
        self._medium = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._depth = Default::default();
        self._alt = Default::default();
        self._sog = Default::default();
        self._cog = Default::default();
        self._msgs = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        41
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._name.len() + 2;
        message_list_serialization_size!(dyn_size, self._msgs);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_f64_le(self._report_time);
        bfr.put_u8(self._medium);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._depth);
        bfr.put_f32_le(self._alt);
        bfr.put_f32_le(self._sog);
        bfr.put_f32_le(self._cog);
        serialize_message_list!(bfr, self._msgs);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);
        self._report_time = bfr.get_f64_le();
        self._medium = bfr.get_u8();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._depth = bfr.get_f32_le();
        self._alt = bfr.get_f32_le();
        self._sog = bfr.get_f32_le();
        self._cog = bfr.get_f32_le();
        self._msgs = deserialize_message_list(bfr)?;
        Ok(())
    }
}
