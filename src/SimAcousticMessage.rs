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

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Acknowledgement
    pub const SAM_ACK: u32 = 0x01;
    /// Delayed
    pub const SAM_DELAYED: u32 = 0x02;
    /// Reply
    pub const SAM_REPLY: u32 = 0x03;
}

/// Send an acoustic message.
#[derive(Default, Clone)]
pub struct SimAcousticMessage {
    /// Message Header
    pub _header: Header,
    /// Absolute latitude of sending vehicle.
    pub _lat: f64,
    /// Absolute longitude of sending vehicle.
    pub _lon: f64,
    /// Depth of sending vehicle.
    pub _depth: f32,
    /// Sentence string sent/received by the modem
    pub _sentence: String,
    /// Transmission time.
    pub _txtime: f64,
    /// The modem being used.
    pub _modem_type: String,
    /// Name of source system.
    pub _sys_src: String,
    pub _seq: u16,
    pub _sys_dst: String,
    pub _flags: u8,
    pub _data: Vec<u8>,
}

impl Message for SimAcousticMessage {
    fn new() -> SimAcousticMessage {
        

        SimAcousticMessage {
            _header: Header::new(207),
            _lat: Default::default(),
            _lon: Default::default(),
            _depth: Default::default(),
            _sentence: Default::default(),
            _txtime: Default::default(),
            _modem_type: Default::default(),
            _sys_src: Default::default(),
            _seq: Default::default(),
            _sys_dst: Default::default(),
            _flags: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        207
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        207
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

    fn clear(&mut self) {
        self._header = Header::new(207);
        self._lat = Default::default();
        self._lon = Default::default();
        self._depth = Default::default();
        self._sentence = Default::default();
        self._txtime = Default::default();
        self._modem_type = Default::default();
        self._sys_src = Default::default();
        self._seq = Default::default();
        self._sys_dst = Default::default();
        self._flags = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        31
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sentence.len() + 2;
        dyn_size += self._modem_type.len() + 2;
        dyn_size += self._sys_src.len() + 2;
        dyn_size += self._sys_dst.len() + 2;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._depth);
        serialize_bytes!(bfr, self._sentence.as_bytes());
        bfr.put_f64_le(self._txtime);
        serialize_bytes!(bfr, self._modem_type.as_bytes());
        serialize_bytes!(bfr, self._sys_src.as_bytes());
        bfr.put_u16_le(self._seq);
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._depth = bfr.get_f32_le();
        deserialize_string!(bfr, self._sentence);
        self._txtime = bfr.get_f64_le();
        deserialize_string!(bfr, self._modem_type);
        deserialize_string!(bfr, self._sys_src);
        self._seq = bfr.get_u16_le();
        deserialize_string!(bfr, self._sys_dst);
        self._flags = bfr.get_u8();
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
