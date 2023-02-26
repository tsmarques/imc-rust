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
use crate::DUNE_IMC_CONST_NULL_ID;

/// Communication Mean
#[allow(non_camel_case_types)]
pub enum CommunicationMeanEnum {
    /// WiFi
    CMEAN_WIFI = 0,
    /// Acoustic
    CMEAN_ACOUSTIC = 1,
    /// Satellite
    CMEAN_SATELLITE = 2,
    /// GSM
    CMEAN_GSM = 3,
    /// Any
    CMEAN_ANY = 4,
    /// All
    CMEAN_ALL = 5,
}

/// Data Mode
#[allow(non_camel_case_types)]
pub enum DataModeEnum {
    /// Inline Message
    DMODE_INLINEMSG = 0,
    /// Text
    DMODE_TEXT = 1,
    /// Raw Data
    DMODE_RAW = 2,
    /// Abort
    DMODE_ABORT = 3,
    /// Range
    DMODE_RANGE = 4,
    /// Reverse Range
    DMODE_REVERSE_RANGE = 5,
}

/// Request data to be sent over a specified communication mean.
#[derive(Default, Clone)]
pub struct TransmissionRequest {
    /// Message Header
    pub _header: Header,
    /// The unique identifier for this request.
    pub _req_id: u16,
    /// Communication mean to be used to transfer these data.
    pub _comm_mean: u8,
    /// The name of the system where to send this message.
    pub _destination: String,
    /// Deadline for message transmission (seconds since epoch).
    pub _deadline: f64,
    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _range: f32,
    /// Type of data to be transmitted.
    /// Abort and Range mode can only be used with comm_mean=ACOUSTIC
    pub _data_mode: u8,
    /// Data to be transmitted if selected *data_mode* is *INLINEMSG*.
    pub _msg_data: Option<Box<dyn Message>>,
    /// Data to be transmitted if selected *data_mode* is *TEXT*.
    pub _txt_data: String,
    /// Data to be transmitted if selected *data_mode* is *RAW*.
    pub _raw_data: Vec<u8>,
}

impl Message for TransmissionRequest {
    fn new() -> TransmissionRequest {
        

        TransmissionRequest {
            _header: Header::new(515),
            _req_id: Default::default(),
            _comm_mean: Default::default(),
            _destination: Default::default(),
            _deadline: Default::default(),
            _range: Default::default(),
            _data_mode: Default::default(),
            _msg_data: Default::default(),
            _txt_data: Default::default(),
            _raw_data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        515
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        515
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
        if let Some(m) = &mut self._msg_data {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._msg_data {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._msg_data {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._msg_data {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._msg_data {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(515);
        self._req_id = Default::default();
        self._comm_mean = Default::default();
        self._destination = Default::default();
        self._deadline = Default::default();
        self._range = Default::default();
        self._data_mode = Default::default();
        self._msg_data = Default::default();
        self._txt_data = Default::default();
        self._raw_data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._destination.len() + 2;
        inline_message_serialization_size!(dyn_size, self._msg_data);
        dyn_size += self._txt_data.len() + 2;
        dyn_size += self._raw_data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u8(self._comm_mean);
        serialize_bytes!(bfr, self._destination.as_bytes());
        bfr.put_f64_le(self._deadline);
        bfr.put_f32_le(self._range);
        bfr.put_u8(self._data_mode);
        serialize_inline_message!(bfr, self._msg_data);
        serialize_bytes!(bfr, self._txt_data.as_bytes());
        serialize_bytes!(bfr, self._raw_data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._comm_mean = bfr.get_u8();
        deserialize_string!(bfr, self._destination);
        self._deadline = bfr.get_f64_le();
        self._range = bfr.get_f32_le();
        self._data_mode = bfr.get_u8();
        self._msg_data = deserialize_inline(bfr).ok();
        deserialize_string!(bfr, self._txt_data);
        deserialize_bytes!(bfr, self._raw_data);
        Ok(())
    }
}
