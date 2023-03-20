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

#[derive(Default, Clone)]
pub struct EntityMonitoringState {
    /// Message Header
    pub _header: Header,
    /// Number of entitities being monitored.
    pub _mcount: u8,
    /// Comma separated list of all entity names being monitored.
    pub _mnames: String,
    /// Number of entitities with non-critical errors.
    pub _ecount: u8,
    /// Comma separated list of all entity names with non-critical
    /// errors.
    pub _enames: String,
    /// Number of entitities with critical errors.
    pub _ccount: u8,
    /// Comma separated list of all entity names with critical errors.
    pub _cnames: String,
    /// Description of last error.
    pub _last_error: String,
    /// Time of last error (Epoch time).
    pub _last_error_time: f64,
}

impl Message for EntityMonitoringState {
    fn new() -> EntityMonitoringState {
        EntityMonitoringState {
            _header: Header::new(503),
            _mcount: Default::default(),
            _mnames: Default::default(),
            _ecount: Default::default(),
            _enames: Default::default(),
            _ccount: Default::default(),
            _cnames: Default::default(),
            _last_error: Default::default(),
            _last_error_time: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        503
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        503
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
        self._header = Header::new(503);
        self._mcount = Default::default();
        self._mnames = Default::default();
        self._ecount = Default::default();
        self._enames = Default::default();
        self._ccount = Default::default();
        self._cnames = Default::default();
        self._last_error = Default::default();
        self._last_error_time = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        11
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._mnames.len() + 2;
        dyn_size += self._enames.len() + 2;
        dyn_size += self._cnames.len() + 2;
        dyn_size += self._last_error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._mcount);
        serialize_bytes!(bfr, self._mnames.as_bytes());
        bfr.put_u8(self._ecount);
        serialize_bytes!(bfr, self._enames.as_bytes());
        bfr.put_u8(self._ccount);
        serialize_bytes!(bfr, self._cnames.as_bytes());
        serialize_bytes!(bfr, self._last_error.as_bytes());
        bfr.put_f64_le(self._last_error_time);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._mcount = bfr.get_u8();
        deserialize_string!(bfr, self._mnames);
        self._ecount = bfr.get_u8();
        deserialize_string!(bfr, self._enames);
        self._ccount = bfr.get_u8();
        deserialize_string!(bfr, self._cnames);
        deserialize_string!(bfr, self._last_error);
        self._last_error_time = bfr.get_f64_le();
        Ok(())
    }
}
