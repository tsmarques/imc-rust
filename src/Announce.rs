//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// A system description that is to be broadcasted to other systems.
#[derive(Default)]
pub struct Announce {
    /// Message Header
    pub _header: Header,
    /// System name.
    pub _sys_name: String,
    /// System type.
    pub _sys_type: u8,
    /// The owner IMC system ID.
    pub _owner: u16,
    /// WGS-84 Latitude. If lat=0 and lon=0 means location value is unknown.
    pub _lat: f64,
    /// WGS-84 Longitude. If lat=0 and lon=0 means location value is unknown.
    pub _lon: f64,
    /// Height above WGS-84 ellipsoid.
    pub _height: f32,
    /// Semicolon separated list of URLs. Examples of such URLs are:
    /// - *imc+udp://192.168.106.34:6002/*
    /// - *dune://0.0.0.0/uid/1294925553839635/*
    /// - *http://192.168.106.34/dune/*.
    pub _services: String,
}

impl Message for Announce {
    fn new() -> Announce {
        let msg = Announce {
            _header: Header::new(151),
            _sys_name: Default::default(),
            _sys_type: Default::default(),
            _owner: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _services: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        151
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        151
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(151);
        self._sys_name = Default::default();
        self._sys_type = Default::default();
        self._owner = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._height = Default::default();
        self._services = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        23
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sys_name.len() + 2;
        dyn_size += self._services.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._sys_name.as_bytes());
        bfr.put_u8(self._sys_type);
        bfr.put_u16_le(self._owner);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        serialize_bytes!(bfr, self._services.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._sys_name);
        self._sys_type = bfr.get_u8();
        self._owner = bfr.get_u16_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._height = bfr.get_f32_le();
        deserialize_string!(bfr, self._services);
        Ok(())
    }
}
